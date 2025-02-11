# SEC Data Fetcher

An npm library to fetch SEC data from all supported endpoints with rate limiting, written in TypeScript.

## Features

- **CIK Lookup**: Lookup Central Index Key (CIK) using a company's ticker symbol.
- **Company Data**: Retrieve company submission data.
- **Reports**: Fetch recent filings (e.g., 10-Q, 10-K, 8-K) after a specified date.
- **Company Facts**: Get standardized fundamental data.
- **Parse Filings**: Parse SEC filings from URLs or strings into structured objects.
- **Rate Limiting**: Configurable rate limiting to comply with SEC guidelines.
- **TypeScript Support**: Includes types for better TypeScript integration.
- **Customizable User-Agent**: Set your own User-Agent string as required by the SEC.

## Installation

```bash
npm install sec-data-fetcher
```

## Usage

```typescript
import { SECClient } from 'sec-data-fetcher';

// Initialize the SECClient with your User-Agent
const secClient = new SECClient({
  userAgent: 'Your Company Name contact@yourcompany.com',
});

(async () => {
  // Lookup CIK
  const cik = await secClient.cikLookup('AAPL');
  console.log('CIK:', cik);

  if (cik) {
    // Get Company Data
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
```

# API Reference

## SECClient

Constructor

```typescript
new SECClient(options: SECClientOptions)
```

## Methods

- **cikLookup(ticker: string):** Promise<string | null>
- **getCompanyData(cik: string):** Promise<any>
- **getReports(cik: string, after?:** Date, forms?: string[]): Promise<Filing[]>
- **getCompanyFacts(cik: string):** Promise<any>
- **getObjectFromString(content:** string): FilingObject
- **getObjectFromUrl(url: string):** Promise<FilingObject>
- **fetchFiling(url: string):** Promise<string>  
  Fetches the raw HTML content of an SEC filing from the provided URL.
- **extractTablesFromFilingUrl(url: string):** Promise<Array<Array<Array<string>>>>  
  Fetches the filing content from a URL and extracts tables from the filing.
- **extractTablesFromContent(filingContent: string):** Array<Array<Array<string>>>  
  Extracts tables from the provided raw HTML content of an SEC filing.

### Extract Tables from SEC Filings

The package provides two ways to extract tables from SEC filings:

1. **Extract from a URL**: Fetch and extract tables directly from an SEC filing URL.
2. **Extract from provided HTML content**: Extract tables from the provided HTML content when you already have the filing data.

```typescript
import { SECClient } from 'sec-data-fetcher';

const secClient = new SECClient({
  userAgent: 'Your Company <your-email@example.com>',
});

// Extract tables from an SEC filing URL
const tablesFromUrl = await secClient.extractTablesFromFilingUrl(
  'https://www.sec.gov/...',
);

// Extract tables from provided HTML content
const filingContent = '<html>...</html>';
const tablesFromContent = secClient.extractTablesFromContent(filingContent);
```

## Notes

- Ensure that you comply with the SEC’s Terms of Use.
- All requests include the required headers (User-Agent, Accept-Encoding, and Host).
- The library includes rate limiting to prevent exceeding the SEC’s request limits.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

MIT License. See the LICENSE file for details.

## Disclaimer

This library is provided as-is and is not affiliated with or endorsed by the U.S. Securities and Exchange Commission. Use this library responsibly and in compliance with the SEC’s Terms of Use.
