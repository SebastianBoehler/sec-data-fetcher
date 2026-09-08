import assert from 'node:assert/strict';
import { SECClient } from '../dist/index.js';

const userAgent = process.env.SEC_USER_AGENT;
if (!userAgent)
  throw new Error(
    'Set SEC_USER_AGENT to your application name and real contact email.',
  );
const client = new SECClient({ userAgent, maxRequestsPerSecond: 2 });
const started = performance.now();
try {
  const cik = await client.cikLookup('AAPL');
  assert.equal(cik, '0000320193');
  const company = await client.getCompanyData(cik);
  assert.equal(company.cik, cik);
  assert.equal(company.name, 'Apple Inc.');
  const facts = await client.getCompanyFacts(cik);
  assert.equal(facts.cik, Number(cik));
  assert(Object.keys(facts.facts).length > 0);
  const recent = company.filings.recent;
  const index = recent.form.findIndex(
    (form) => form === '10-K' || form === '10-Q',
  );
  assert(index >= 0, 'No recent 10-K or 10-Q returned by SEC.');
  const accession = recent.accessionNumber[index];
  const url = `https://www.sec.gov/Archives/edgar/data/${Number(cik)}/${accession.replaceAll('-', '')}/${recent.primaryDocument[index]}`;
  const content = await client.fetchFiling(url);
  assert(content.length > 0);
  const tables = client.extractTablesFromContent(content);
  assert(tables.length > 0, 'Filing has no extracted tables.');
  console.log(
    JSON.stringify(
      {
        company: company.name,
        cik,
        accession,
        url,
        filingBytes: Buffer.byteLength(content),
        tables: tables.length,
        elapsedSeconds: Number(
          ((performance.now() - started) / 1000).toFixed(2),
        ),
        peakRssMiB: Math.round(process.resourceUsage().maxRSS / 1024),
      },
      null,
      2,
    ),
  );
} catch (error) {
  console.error(`Live SEC check failed: ${error.message}`);
  if (error.response?.status)
    console.error(
      `HTTP ${error.response.status}; check SEC fair access, contact User-Agent and network/IP access.`,
    );
  process.exitCode = 1;
}
