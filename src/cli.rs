use clap::{Args, Parser, Subcommand};
use sec_data_fetcher::{Cik, NaiveDate};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Fetch SEC EDGAR filings and company facts; parse local HTML and XML"
)]
pub struct Arguments {
    #[arg(long, env = "SEC_USER_AGENT", global = true, hide_env_values = true)]
    pub user_agent: Option<String>,
    #[arg(long, default_value_t = 10, global = true, value_parser = clap::value_parser!(u32).range(1..=10))]
    pub requests_per_second: u32,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Look up a company's padded CIK by ticker.
    Lookup { ticker: String },
    /// Get company submissions metadata, including recent filing columns.
    Submissions { cik: Cik },
    /// Get standardized XBRL facts with units and filing provenance.
    Facts { cik: Cik },
    /// Select recent filing metadata without downloading document bodies.
    Reports {
        cik: Cik,
        #[arg(long)]
        after: NaiveDate,
        #[arg(long, value_delimiter = ',', default_value = "10-K,10-Q,8-K", value_parser = clap::builder::NonEmptyStringValueParser::new())]
        forms: Vec<String>,
    },
    /// Download a single SEC document to stdout.
    Fetch { url: String },
    /// Extract HTML table cell text as JSON.
    Tables(Input),
    /// Parse XML into ordered elements, attributes and text as JSON.
    Xml(Input),
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Input {
    #[arg(long)]
    pub file: Option<PathBuf>,
    #[arg(long)]
    pub url: Option<String>,
}
