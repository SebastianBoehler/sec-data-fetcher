import { SECClient } from '../src/secAPI';

describe('SECClient', () => {
  const secClient = new SECClient({
    userAgent: 'Your Company Name <contact@company.domain>',
  });

  it('should lookup CIK for a valid ticker', async () => {
    const cik = await secClient.cikLookup('AAPL');
    expect(cik).toBe('0000320193'); // Apple's CIK
  });

  it('should return null for an invalid ticker', async () => {
    const cik = await secClient.cikLookup('INVALIDTICKER');
    expect(cik).toBeNull();
  });

  it('should fetch company data', async () => {
    const cik = '0000320193';
    const data = await secClient.getCompanyData(cik);
    expect(data).toHaveProperty('cik', '0000320193');
  });

  it('should parse filing content from a string', () => {
    const content = `<SEC-DOCUMENT>...</SEC-DOCUMENT>`;
    const parsed = secClient.getObjectFromString(content);
    expect(parsed).toHaveProperty('SEC-DOCUMENT');
  });

  it('should fetch and extract tables from a filing URL', async () => {
    // Mock filing URL from SEC (you can use a real one or mock the response)
    const filingUrl =
      'https://www.sec.gov/Archives/edgar/data/0000320193/0000320193-21-000010.txt';

    const tables = await secClient.extractTablesFromFilingUrl(filingUrl);
    expect(tables.length).toBeGreaterThan(0); // Ensure at least one table is extracted
  });

  it('should extract tables from provided filing content', () => {
    const filingContent = `
      <html>
        <body>
          <table>
            <tr><th>Header 1</th><th>Header 2</th></tr>
            <tr><td>Row 1 Col 1</td><td>Row 1 Col 2</td></tr>
            <tr><td>Row 2 Col 1</td><td>Row 2 Col 2</td></tr>
          </table>
        </body>
      </html>
    `;

    const tables = secClient.extractTablesFromContent(filingContent);
    expect(tables.length).toBe(1);
    expect(tables[0][0]).toEqual(['Header 1', 'Header 2']);
    expect(tables[0][1]).toEqual(['Row 1 Col 1', 'Row 1 Col 2']);
    expect(tables[0][2]).toEqual(['Row 2 Col 1', 'Row 2 Col 2']);
  });
});
