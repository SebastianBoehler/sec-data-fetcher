<p align="center">
  <img src="https://raw.githubusercontent.com/SebastianBoehler/sec-data-fetcher/main/docs/assets/banner.svg" alt="SEC Data Fetcher — EDGAR filings, company facts and tables for TypeScript and Node.js" width="100%" />
</p>

<p align="center">
  <a href="https://github.com/SebastianBoehler/sec-data-fetcher/actions/workflows/test.yml"><img src="https://github.com/SebastianBoehler/sec-data-fetcher/actions/workflows/test.yml/badge.svg" alt="CI" /></a>
  <a href="https://www.npmjs.com/package/sec-data-fetcher"><img src="https://img.shields.io/npm/v/sec-data-fetcher" alt="npm version" /></a>
  <a href="https://nodejs.org/"><img src="https://img.shields.io/badge/node-%3E%3D22-417e38" alt="Node.js 22 or newer" /></a>
  <a href="https://github.com/SebastianBoehler/sec-data-fetcher/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT license" /></a>
</p>

A **SEC EDGAR filings and financial-data toolkit** for Node.js. Find companies by ticker, retrieve submission history and standardized XBRL facts, download filings, parse XML and extract HTML tables. Use it in research scripts, financial-data pipelines and backend applications.

No API key or paid service is required for the supported public SEC endpoints. Supply your application's name and a real contact email in the User-Agent.

## Install

```sh
npm install sec-data-fetcher
```

Requires **Node.js 22+**. CI tests Node 22, 24 and 26, including CommonJS, ESM and TypeScript consumers installed from the npm tarball. Upgrading from 1.x? Read the [migration notes](https://github.com/SebastianBoehler/sec-data-fetcher/blob/main/CHANGELOG.md).

## From ticker to filings

Save as `filings.mjs`:

```js
import { SECClient } from 'sec-data-fetcher';

const userAgent = process.env.SEC_USER_AGENT;
if (!userAgent)
  throw new Error('Set SEC_USER_AGENT to your app name and contact email.');

const client = new SECClient({ userAgent, maxRequestsPerSecond: 2 });
const cik = await client.cikLookup('AAPL');
if (!cik) throw new Error('Ticker not found.');

const company = await client.getCompanyData(cik);
const recent = company.filings.recent;
console.log(company.name, company.cik);
console.table(
  recent.form.slice(0, 5).map((form, index) => ({
    form,
    filed: recent.filingDate[index],
    accession: recent.accessionNumber[index],
  })),
);
```

```sh
SEC_USER_AGENT='YourApp you@your-domain.com' node filings.mjs
```

Use your own contact information. This example fetches metadata only; it does not download every filing. In CommonJS, use `const { SECClient } = require('sec-data-fetcher')` inside your script.

## Financial facts

Using the same `client` and `cik`:

```ts
const facts = await client.getCompanyFacts(cik);
const assets = facts.facts['us-gaap']?.Assets?.units.USD;
if (!assets)
  throw new Error('No US-GAAP Assets facts in USD for this company.');
console.table(
  assets.slice(-5).map(({ val, end, filed, form, accn }) => ({
    value: val,
    periodEnd: end,
    filed,
    form,
    accession: accn,
  })),
);
```

These are the final five entries in the SEC response, **not a deduplicated time series**. The same reporting period can appear in multiple filings or amendments. Keep units, period dates and accession numbers when selecting facts. See the [SEC XBRL API documentation](https://www.sec.gov/search-filings/edgar-application-programming-interfaces) for coverage and context.

## Download a filing and extract tables

```ts
const index = recent.form.findIndex(
  (form) => form === '10-K' || form === '10-Q',
);
if (index < 0) throw new Error('No recent annual or quarterly report.');
const accession = recent.accessionNumber[index].replaceAll('-', '');
const url = `https://www.sec.gov/Archives/edgar/data/${Number(cik)}/${accession}/${recent.primaryDocument[index]}`;

const html = await client.fetchFiling(url);
const tables = client.extractTablesFromContent(html);
console.log(`${tables.length} tables extracted from ${url}`);
```

Tables are arrays of rows of cell text (`string[][][]`). Nested tables are returned separately; a parent cell's text still includes its nested content. Rowspan/colspan, financial units and statement semantics are not reconstructed. For XML documents, `getObjectFromString(xml)` returns a generic XML object; it is not an iXBRL or SGML financial-statement parser.

## API

```ts
const client = new SECClient({
  userAgent: 'YourApp you@your-domain.com', // required; use your real contact
  maxRequestsPerSecond: 2, // optional; integer 1–10, default 10
});
```

| Method                            | Result                        | Behavior                                                                     |
| --------------------------------- | ----------------------------- | ---------------------------------------------------------------------------- |
| `cikLookup(ticker)`               | `Promise<string \| null>`     | Case-insensitive ticker lookup; padded CIK or `null`.                        |
| `getCompanyData(cik)`             | `Promise<CompanySubmissions>` | SEC submissions metadata with a normalized string CIK.                       |
| `getCompanyFacts(cik)`            | `Promise<CompanyFacts>`       | SEC XBRL company facts; retains SEC's numeric CIK.                           |
| `getReports(cik, after?, forms?)` | `Promise<Filing[]>`           | Filters recent filings and downloads every matching document into `content`. |
| `fetchFiling(url)`                | `Promise<string>`             | Raw document text.                                                           |
| `getObjectFromString(xml)`        | `FilingObject`                | Generic XML parsing; document-defined entities are not expanded.             |
| `getObjectFromUrl(url)`           | `Promise<FilingObject>`       | Downloads text and applies the XML parser.                                   |
| `extractTablesFromContent(html)`  | `string[][][]`                | Extracts HTML table cell text.                                               |
| `extractTablesFromFilingUrl(url)` | `Promise<string[][][]>`       | Downloads a document and extracts its tables.                                |

CIKs are strings containing 1–10 digits; leading zeros are added automatically. Core response types are exported from `sec-data-fetcher`. Additional submissions fields are preserved as `unknown`.

`getReports` defaults to `after = new Date('2024-01-01')` and forms `['10-Q', '10-K', '8-K']` for compatibility. The date comparison is strictly **after**. Pass a narrow date range and explicit forms, including amendments such as `10-K/A` if needed. It reads only `filings.recent`; it does not traverse historical files in `filings.files`. All matching content is held in memory. For larger workloads, use metadata and download individual documents as needed.

## SEC access and errors

- Run on a Node.js backend. SEC data APIs do not support browser CORS.
- The SEC limit is **10 requests/second per user across all machines**. The library's limiter is per client instance; share a client and coordinate budgets across workers.
- Requests have a 30-second timeout. HTTP errors propagate with Axios status information. There are no automatic retries or alternate data sources.
- `403` may reflect User-Agent, network/IP or SEC access restrictions; `429` means the request rate needs attention. A passed CI run does not guarantee access from every deployment network.
- Invalid CIKs, empty User-Agents and rates outside 1–10 fail before requests. Parsing HTML as generic XML does not produce reliable financial statements.

See the SEC's [fair-access guidance](https://www.sec.gov/about/developer-resources). For full-history bulk ingestion, use the SEC's official bulk archives instead of issuing one request per company.

## Contribute and maintain

Bug reports with a public accession number or reproducible fixture are especially useful. See [CONTRIBUTING.md](https://github.com/SebastianBoehler/sec-data-fetcher/blob/main/CONTRIBUTING.md), the [security policy](https://github.com/SebastianBoehler/sec-data-fetcher/blob/main/SECURITY.md) and [changelog](https://github.com/SebastianBoehler/sec-data-fetcher/blob/main/CHANGELOG.md).

```sh
npm ci
npm run check      # lint, deterministic tests, packed-package consumer checks
SEC_USER_AGENT='YourApp you@your-domain.com' npm run test:live
```

The live smoke makes four requests: ticker mapping, submissions, company facts and one annual/quarterly filing. It checks table extraction and reports time/memory for that run. It is separate from the deterministic PR checks because SEC access can vary by network. Dependabot proposes dependency updates; a weekly live workflow checks upstream compatibility.

## License

[MIT](https://github.com/SebastianBoehler/sec-data-fetcher/blob/main/LICENSE) © 2024–2026 Sebastian Boehler. Independent open-source software; not affiliated with or endorsed by the SEC.
