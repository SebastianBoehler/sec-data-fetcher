import { SECClient } from './secAPI'; // Adjust the import path as needed

// Initialize the SECClient with your User-Agent
const secClient = new SECClient({
  userAgent: 'Company Name <contact@company.domain>',
});

// Example usage
(async () => {
  // Lookup CIK
  const cik = await secClient.cikLookup('AAPL');
  console.log('CIK:', cik);

  // Get Company Data
  if (cik) {
    const companyData = await secClient.getCompanyData(cik);
    console.log('Company Data:', companyData);

    // Get Reports
    const reports = await secClient.getReports(cik);
    console.log('Reports:', reports);

    // Parse a filing from URL
    if (reports.length > 0) {
      const filingUrl = `https://www.sec.gov/Archives/edgar/data/${parseInt(
        cik,
        10,
      )}/${reports[0].accessionNumber.replace(/-/g, '')}/${
        reports[0].primaryDocument
      }`;
      const filingObject = await secClient.getObjectFromUrl(filingUrl);
      console.log('Filing Object:', filingObject);
    }
  }
})();
