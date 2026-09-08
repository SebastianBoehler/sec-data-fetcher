//! Fetch SEC EDGAR data with one shared request budget and parse local filings.
//!
//! ```no_run
//! use sec_data_fetcher::{Cik, SecClient};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = SecClient::new("ResearchApp contact@example.com")?;
//! let company = client.get_company_data(&Cik::new("320193")?).await?;
//! println!("{}", company.name);
//! # Ok(()) }
//! ```
mod cik;
mod client;
mod error;
mod facts;
mod filing;
mod limiter;
pub mod parsers;
mod submissions;
mod tickers;

pub use chrono::NaiveDate;
pub use cik::Cik;
pub use client::SecClient;
pub use error::{Error, Result};
pub use facts::{CompanyFacts, FactConcept, FactValue, XbrlFact};
pub use filing::Filing;
pub use parsers::{Tables, XmlAttribute, XmlElement, XmlNode, extract_tables, parse_xml};
pub use submissions::{CompanySubmissions, Filings, HistoricalFile, RecentFilings};
