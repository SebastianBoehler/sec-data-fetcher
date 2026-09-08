import { SECClient } from './index';

async function main() {
  const userAgent = process.env.SEC_USER_AGENT;
  if (!userAgent)
    throw new Error(
      'Set SEC_USER_AGENT to your application name and contact email.',
    );
  const client = new SECClient({ userAgent, maxRequestsPerSecond: 2 });
  const cik = await client.cikLookup(process.argv[2] ?? 'AAPL');
  if (!cik) throw new Error('Ticker not found.');
  const company = await client.getCompanyData(cik);
  const recent = company.filings.recent;
  console.log(`${company.name} (CIK ${company.cik})`);
  console.table(
    recent.form.slice(0, 5).map((form, index) => ({
      form,
      filed: recent.filingDate[index],
      accession: recent.accessionNumber[index],
    })),
  );
}

main().catch((error: unknown) => {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});
