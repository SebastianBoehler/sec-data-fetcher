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
});
