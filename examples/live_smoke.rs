use sec_data_fetcher::{NaiveDate, SecClient, extract_tables};
use std::{error::Error, time::Instant};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_agent = std::env::var("SEC_USER_AGENT")?;
    let client = SecClient::with_rate_limit(user_agent, 2)?;
    let start = Instant::now();
    let cik = client.cik_lookup("AAPL").await?.ok_or("AAPL not found")?;
    if cik.as_str() != "0000320193" {
        return Err("Unexpected AAPL CIK".into());
    }
    let company = client.get_company_data(&cik).await?;
    if company.name != "Apple Inc." || company.cik != cik {
        return Err("Unexpected submissions identity".into());
    }
    let facts = client.get_company_facts(&cik).await?;
    if facts.cik != cik || facts.facts.is_empty() {
        return Err("Unexpected company facts".into());
    }
    let reports = company.filings.recent.select(
        &cik,
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        &["10-K", "10-Q"],
    )?;
    let filing = reports
        .first()
        .ok_or("No recent annual or quarterly filing")?;
    let url = filing.url()?;
    let content = client.fetch_filing(&url).await?;
    let tables = extract_tables(&content);
    if tables.is_empty() {
        return Err("No filing tables extracted".into());
    }
    println!(
        "{}",
        serde_json::json!({ "company": company.name, "cik": cik,
        "accession": filing.accession_number, "url": url, "filingBytes": content.len(),
        "tables": tables.len(), "elapsedSeconds": start.elapsed().as_secs_f64() })
    );
    Ok(())
}
