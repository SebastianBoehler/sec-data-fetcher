# Version 3: Rust migration

Version 3 replaces the TypeScript source, npm build and Node CI with a Rust library and CLI. There is no JavaScript compatibility layer. Existing npm installations remain on the independently published 2.0.0 artifact; upgrading means integrating Rust or calling the executable.

| Previous npm API | Rust API |
| --- | --- |
| `SECClient({ userAgent, maxRequestsPerSecond })` | `SecClient::with_rate_limit(user_agent, rate)` |
| `cikLookup` | `cik_lookup` |
| `getCompanyData` / `getCompanyFacts` | `get_company_data` / `get_company_facts` |
| `getReports` | `get_reports` (metadata only) |
| `fetchFiling` | `fetch_filing` |
| `extractTablesFromContent` | Free function `extract_tables` |
| `getObjectFromString` | Free function `parse_xml` |
| `extractTablesFromFilingUrl` / `getObjectFromUrl` | `extract_tables_from_filing_url` / `get_object_from_url` |

Behavior changes:

- CIKs are validated `Cik` values and serialize as padded strings in both submissions and facts.
- Library filing selection requires explicit date and forms. The CLI requires `--after`, with default forms `10-K,10-Q,8-K`. Selection no longer downloads all matching bodies; call `Filing::url()` and `fetch_filing` for each selected document.
- XML output is an ordered element/text tree. Text is not converted into numbers; leading zeros survive. Namespace URIs are retained, DTDs rejected and depth bounded.
- Downloads require HTTPS SEC hosts, reject redirects, and report all non-2xx statuses. Clients and clones share pacing; independent clients still need coordination.
- Node.js is no longer required at runtime. Building needs Rust and native TLS build prerequisites.

The API still does not traverse historical submission shards, reconstruct table spans, deduplicate financial facts or semantically parse iXBRL/SGML. See the README for supported behavior and examples.
