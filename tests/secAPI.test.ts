import { SECClient } from '../src';
import { createHttpClient } from '../src/config';

jest.mock('../src/config', () => ({
  ...jest.requireActual('../src/config'),
  createHttpClient: jest.fn(),
}));

const get = jest.fn();
let client: SECClient;
beforeEach(() => {
  get.mockReset();
  (createHttpClient as jest.Mock).mockReturnValue({ get });
  client = new SECClient({ userAgent: 'Test suite test@example.com' });
});

it('normalizes a ticker and returns a padded CIK without cache busting', async () => {
  get.mockResolvedValue({
    status: 200,
    data: {
      fields: ['cik', 'name', 'ticker', 'exchange'],
      data: [[320193, 'Apple', 'AAPL', 'Nasdaq']],
    },
  });
  await expect(client.cikLookup(' aapl ')).resolves.toBe('0000320193');
  expect(get.mock.calls[0][0]).toBe(
    'https://www.sec.gov/files/company_tickers_exchange.json',
  );
  await expect(client.cikLookup('UNKNOWN')).resolves.toBeNull();
});

it('pads CIKs before fetching submissions and preserves the normalized output', async () => {
  get.mockResolvedValue({ data: { cik: 320193, name: 'Apple' } });
  await expect(client.getCompanyData('320193')).resolves.toEqual({
    cik: '0000320193',
    name: 'Apple',
  });
  expect(get.mock.calls[0][0]).toBe(
    'https://data.sec.gov/submissions/CIK0000320193.json',
  );
  expect(get.mock.calls[0][1].headers['User-Agent']).toBe(
    'Test suite test@example.com',
  );
});

it('pads CIKs for company facts and retains the SEC fact response', async () => {
  const facts = { cik: 320193, facts: {} };
  get.mockResolvedValue({ data: facts });
  await expect(client.getCompanyFacts('320193')).resolves.toBe(facts);
  expect(get.mock.calls[0][0]).toBe(
    'https://data.sec.gov/api/xbrl/companyfacts/CIK0000320193.json',
  );
});

it.each(['', 'abc', '../foo', '12345678901'])(
  'rejects invalid CIK %j before a request',
  async (cik) => {
    await expect(client.getCompanyData(cik)).rejects.toThrow('CIK');
    expect(get).not.toHaveBeenCalled();
  },
);

it.each([0, -1, 11, 1.5, NaN])('rejects invalid rate %s', (rate) => {
  expect(
    () =>
      new SECClient({
        userAgent: 'Test test@example.com',
        maxRequestsPerSecond: rate,
      }),
  ).toThrow('1 to 10');
});

it('requires a nonempty user agent', () => {
  expect(() => new SECClient({ userAgent: '  ' })).toThrow('userAgent');
});

it('filters reports before downloading and constructs the SEC archive URL', async () => {
  get
    .mockResolvedValueOnce({
      data: {
        cik: 320193,
        filings: {
          recent: {
            form: ['10-K', '8-K', '4'],
            filingDate: ['2026-01-02', '2026-01-01', '2026-01-03'],
            accessionNumber: ['0000320193-26-000001', 'old', 'excluded'],
            primaryDocument: ['aapl.htm', 'old.htm', 'excluded.xml'],
            isXBRL: [1, 0, 0],
            act: ['34', '34', '34'],
            primaryDocDescription: ['Annual report', '', ''],
          },
        },
      },
    })
    .mockResolvedValueOnce({ data: '<html>report</html>' });
  const reports = await client.getReports('320193', new Date('2026-01-01'), [
    '10-K',
    '8-K',
  ]);
  expect(reports).toHaveLength(1);
  expect(reports[0]).toMatchObject({
    cik: '0000320193',
    form: '10-K',
    content: '<html>report</html>',
  });
  expect(get.mock.calls[1][0]).toBe(
    'https://www.sec.gov/Archives/edgar/data/320193/000032019326000001/aapl.htm',
  );
  expect(get).toHaveBeenCalledTimes(2);
});

it('rejects invalid dates before downloading', async () => {
  await expect(
    client.getReports('320193', new Date('invalid')),
  ).rejects.toThrow('valid Date');
  expect(get).not.toHaveBeenCalled();
});

it('preserves request errors so callers can inspect HTTP status', async () => {
  const error = Object.assign(new Error('Forbidden'), {
    response: { status: 403 },
  });
  get.mockRejectedValue(error);
  await expect(
    client.fetchFiling('https://www.sec.gov/Archives/filing.htm'),
  ).rejects.toBe(error);
});

it('requests raw filing text with the URL host and parses XML from a URL', async () => {
  get.mockResolvedValue({ data: '<filing><value>42</value></filing>' });
  await expect(
    client.getObjectFromUrl('https://www.sec.gov/Archives/filing.xml'),
  ).resolves.toEqual({ filing: { value: 42 } });
  expect(get.mock.calls[0][1]).toMatchObject({
    responseType: 'text',
    headers: { Host: 'www.sec.gov' },
  });
});

it('does not expand document-defined XML entities', () => {
  const parsed = client.getObjectFromString(
    '<!DOCTYPE filing [<!ENTITY payload "expanded">]><filing>&payload;</filing>',
  );
  expect(parsed).toEqual({ filing: '&payload;' });
});

it('extracts table headers and rows', () => {
  expect(
    client.extractTablesFromContent(
      '<table><tr><th>Metric</th><th>Value</th></tr><tr><td>Cash</td><td>42</td></tr></table>',
    ),
  ).toEqual([
    [
      ['Metric', 'Value'],
      ['Cash', '42'],
    ],
  ]);
});

it('does not duplicate nested table rows or cells in the parent table', () => {
  expect(
    client.extractTablesFromContent(
      '<table><tr><td>Parent<table><tr><td>Child</td></tr></table></td><td>Value</td></tr></table>',
    ),
  ).toEqual([[['ParentChild', 'Value']], [['Child']]]);
});

it('extracts tables from a downloaded filing', async () => {
  get.mockResolvedValue({ data: '<table><tr><td>42</td></tr></table>' });
  await expect(
    client.extractTablesFromFilingUrl(
      'https://www.sec.gov/Archives/filing.htm',
    ),
  ).resolves.toEqual([[['42']]]);
});
