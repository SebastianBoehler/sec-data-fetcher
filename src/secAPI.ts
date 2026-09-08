// src/secAPI.ts

import { AxiosInstance } from 'axios';
import { createHttpClient, getHeaders } from './config';
import { XMLParser } from 'fast-xml-parser';
import { padCik } from './utils';
import { extractTables } from './parsers/tableParser';

import type {
  CompanyTicker,
  CompanySubmissions,
  CompanyFacts,
  Filing,
  FilingObject,
  SECClientOptions,
} from './types';
export type {
  CompanyTicker,
  CompanySubmissions,
  CompanyFacts,
  Filing,
  FilingObject,
  SECClientOptions,
} from './types';

export class SECClient {
  private http: AxiosInstance;
  private userAgent: string;

  /**
   * Initializes a new instance of SECClient.
   * @param options - Configuration options for the client.
   */
  constructor(options: SECClientOptions) {
    const { userAgent, maxRequestsPerSecond = 10 } = options;
    if (typeof userAgent !== 'string' || !userAgent.trim()) {
      throw new Error(
        'userAgent is required; include your application name and contact email.',
      );
    }
    if (
      !Number.isInteger(maxRequestsPerSecond) ||
      maxRequestsPerSecond < 1 ||
      maxRequestsPerSecond > 10
    ) {
      throw new RangeError(
        'maxRequestsPerSecond must be an integer from 1 to 10.',
      );
    }
    this.userAgent = userAgent.trim();
    this.http = createHttpClient(maxRequestsPerSecond, 1000);
  }

  /**
   * Looks up the Central Index Key (CIK) for a given ticker symbol.
   * @param ticker - The company's ticker symbol.
   * @returns The CIK as a string padded to 10 digits, or null if not found.
   */
  public async cikLookup(ticker: string): Promise<string | null> {
    const url = 'https://www.sec.gov/files/company_tickers_exchange.json';
    const response = await this.http.get<{ fields: string[]; data: any[] }>(
      url,
      {
        headers: getHeaders('www.sec.gov', this.userAgent),
      },
    );
    if (response.status !== 200) throw new Error(response.statusText);
    const data = response.data;

    // Map the data into an array of CompanyTicker objects
    const companies: CompanyTicker[] = data.data.map((companyData: any[]) => {
      const company: CompanyTicker = {
        cik: companyData[0],
        name: companyData[1],
        ticker: companyData[2],
        exchange: companyData[3],
      };
      return company;
    });

    const item = companies.find(
      (item: CompanyTicker) =>
        item.ticker.toUpperCase() === ticker.trim().toUpperCase(),
    );

    return item ? String(item.cik).padStart(10, '0') : null;
  }

  /**
   * Retrieves company data from the SEC submissions endpoint.
   * @param cik - The company's Central Index Key.
   * @returns An object containing the company data.
   */
  public async getCompanyData(cik: string): Promise<CompanySubmissions> {
    const url = `https://data.sec.gov/submissions/CIK${padCik(cik)}.json`;
    const response = await this.http.get<CompanySubmissions>(url, {
      headers: getHeaders('data.sec.gov', this.userAgent),
    });

    // Pad the cik in the response data
    response.data.cik = padCik(response.data.cik);

    return response.data;
  }

  /**
   * Fetches recent filings for a company.
   * @param cik - The company's Central Index Key.
   * @param after - Date to filter filings after.
   * @param forms - Array of form types to include.
   * @returns An array of filings.
   */
  public async getReports(
    cik: string,
    after: Date = new Date('2024-01-01'),
    forms: string[] = ['10-Q', '10-K', '8-K'],
  ): Promise<Filing[]> {
    if (Number.isNaN(after.getTime()))
      throw new TypeError('after must be a valid Date.');
    const latestFilings = await this.getCompanyData(cik);
    const { recent } = latestFilings.filings;

    const mapped: Filing[] = recent.form.map((form: string, index: number) => ({
      form,
      cik: padCik(cik),
      primaryDocument: recent.primaryDocument[index],
      filingDate: new Date(recent.filingDate[index]),
      accessionNumber: recent.accessionNumber[index],
      isXBRL: recent.isXBRL[index],
      act: recent.act[index],
      primaryDocDescription: recent.primaryDocDescription[index],
    }));

    const reportAccessions = mapped.filter(
      (filing) => forms.includes(filing.form) && filing.filingDate > after,
    );

    const reports = await Promise.all(
      reportAccessions.map(async (filing) => {
        const accessionNumberNoDashes = filing.accessionNumber.replace(
          /-/g,
          '',
        );
        const cikNumber = parseInt(cik, 10);
        const filingUrl = `https://www.sec.gov/Archives/edgar/data/${cikNumber}/${accessionNumberNoDashes}/${filing.primaryDocument}`;
        const content = await this.fetchFiling(filingUrl);
        return {
          ...filing,
          content,
        };
      }),
    );

    return reports;
  }

  /**
   * Retrieves company facts from the SEC XBRL API.
   * @param cik - The company's Central Index Key.
   * @returns An object containing the company facts.
   */
  public async getCompanyFacts(cik: string): Promise<CompanyFacts> {
    const url = `https://data.sec.gov/api/xbrl/companyfacts/CIK${padCik(cik)}.json`;
    const response = await this.http.get<CompanyFacts>(url, {
      headers: getHeaders('data.sec.gov', this.userAgent),
    });
    return response.data;
  }

  /**
   * Fetches the SEC filing content from a given URL.
   * @param url - The URL of the SEC filing.
   * @returns The raw HTML content of the filing.
   */
  public async fetchFiling(url: string): Promise<string> {
    const response = await this.http.get<string>(url, {
      headers: getHeaders(new URL(url).host, this.userAgent),
      responseType: 'text',
    });
    return response.data;
  }

  /**
   * Extracts tables from a given SEC filing URL.
   * @param url - The URL of the SEC filing.
   * @returns An array of tables extracted from the filing.
   */
  public async extractTablesFromFilingUrl(
    url: string,
  ): Promise<Array<Array<Array<string>>>> {
    const filingContent = await this.fetchFiling(url); // Fetch the raw HTML content
    const tables = extractTables(filingContent); // Extract tables using the parsing function
    return tables;
  }

  /**
   * Extracts tables from the provided HTML content of an SEC filing.
   * @param filingContent - The raw HTML content of the SEC filing.
   * @returns An array of tables extracted from the content.
   */
  public extractTablesFromContent(
    filingContent: string,
  ): Array<Array<Array<string>>> {
    return extractTables(filingContent); // Extract tables from the provided HTML content
  }

  /**
   * Parses SEC filing content from a string into an object.
   * @param content - The SEC filing content as a string.
   * @returns A structured object representing the filing.
   */
  public getObjectFromString(content: string): FilingObject {
    const parser = new XMLParser({
      processEntities: false,
      ignoreDeclaration: true,
      ignoreAttributes: false,
      attributeNamePrefix: '',
      parseTagValue: true,
      trimValues: true,
    });

    const parsed = parser.parse(content);
    return parsed;
  }

  /**
   * Fetches an SEC filing from a URL and parses it into an object.
   * @param url - The URL of the SEC filing.
   * @returns A structured object representing the filing.
   */
  public async getObjectFromUrl(url: string): Promise<FilingObject> {
    return this.getObjectFromString(await this.fetchFiling(url));
  }

  // Additional methods can be added here following the same pattern
}
