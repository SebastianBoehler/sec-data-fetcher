/** SEC's ticker-to-company mapping. */
export interface CompanyTicker {
  cik: number;
  name: string;
  ticker: string;
  exchange: string;
}

/** Columnar recent-filings response from the submissions API. */
export interface RecentFilings {
  form: string[];
  primaryDocument: string[];
  filingDate: string[];
  accessionNumber: string[];
  isXBRL: number[];
  act: string[];
  primaryDocDescription: string[];
  [key: string]: unknown;
}

export interface CompanySubmissions {
  cik: string;
  name: string;
  tickers: string[];
  filings: {
    recent: RecentFilings;
    files: Array<{
      name: string;
      filingCount: number;
      filingFrom: string;
      filingTo: string;
    }>;
  };
  [key: string]: unknown;
}

export interface XbrlFact {
  val: number | string;
  accn: string;
  form: string;
  filed: string;
  start?: string;
  end: string;
  fy?: number;
  fp?: string;
  frame?: string;
}

export interface CompanyFacts {
  cik: number;
  entityName: string;
  facts: Record<
    string,
    Record<
      string,
      {
        label: string;
        description: string;
        units: Record<string, XbrlFact[]>;
      }
    >
  >;
}

export interface Filing {
  form: string;
  cik: string;
  primaryDocument: string;
  filingDate: Date;
  accessionNumber: string;
  isXBRL: number;
  act: string;
  primaryDocDescription: string;
  content?: string;
}

/** Generic XML structure, not interpreted financial statements. */
export type FilingObject = Record<string, unknown>;

export interface SECClientOptions {
  /** Application name and a real contact email, as requested by the SEC. */
  userAgent: string;
  /** Integer from 1 to 10. This limit applies to this client instance only. */
  maxRequestsPerSecond?: number;
}
