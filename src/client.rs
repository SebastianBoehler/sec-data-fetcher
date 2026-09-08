use crate::{
    Cik, CompanyFacts, CompanySubmissions, Error, Filing, Result, Tables, XmlElement,
    extract_tables, limiter::RateLimiter, parse_xml,
};
use chrono::NaiveDate;
use reqwest::{Client, Url, header::HeaderValue};
use serde::de::DeserializeOwned;
use std::{sync::Arc, time::Duration};

/// Clones share a connection pool and a paced request budget. Separate clients do not.
#[derive(Clone)]
pub struct SecClient {
    http: Client,
    limiter: Arc<RateLimiter>,
}

impl SecClient {
    /// Use an application name and real contact email. Default: 10 requests/second.
    pub fn new(user_agent: impl AsRef<str>) -> Result<Self> {
        Self::with_rate_limit(user_agent, 10)
    }

    pub fn with_rate_limit(user_agent: impl AsRef<str>, requests_per_second: u32) -> Result<Self> {
        let user_agent = user_agent.as_ref().trim();
        if user_agent.is_empty() || HeaderValue::from_str(user_agent).is_err() {
            return Err(Error::InvalidInput("User-Agent must identify your application and contact email and contain no control characters".into()));
        }
        if !(1..=10).contains(&requests_per_second) {
            return Err(Error::InvalidInput(
                "requests_per_second must be between 1 and 10".into(),
            ));
        }
        let http = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .build()?;
        Ok(Self {
            http,
            limiter: Arc::new(RateLimiter::new(requests_per_second)),
        })
    }

    pub async fn cik_lookup(&self, ticker: &str) -> Result<Option<Cik>> {
        let ticker = ticker.trim();
        if ticker.is_empty() {
            return Err(Error::InvalidInput("ticker must not be empty".into()));
        }
        let response: crate::tickers::TickerMapping = self
            .json("https://www.sec.gov/files/company_tickers_exchange.json")
            .await?;
        response.lookup(ticker)
    }

    pub async fn get_company_data(&self, cik: &Cik) -> Result<CompanySubmissions> {
        self.json(&format!("https://data.sec.gov/submissions/CIK{cik}.json"))
            .await
    }

    pub async fn get_company_facts(&self, cik: &Cik) -> Result<CompanyFacts> {
        self.json(&format!(
            "https://data.sec.gov/api/xbrl/companyfacts/CIK{cik}.json"
        ))
        .await
    }

    /// Return metadata only. Download selected bodies explicitly to bound memory use.
    pub async fn get_reports(
        &self,
        cik: &Cik,
        after: NaiveDate,
        forms: &[&str],
    ) -> Result<Vec<Filing>> {
        self.get_company_data(cik)
            .await?
            .filings
            .recent
            .select(cik, after, forms)
    }

    /// Fetch one document from an HTTPS SEC URL. Redirects are reported, not followed.
    pub async fn fetch_filing(&self, url: &str) -> Result<String> {
        validate_document_url(url)?;
        Ok(self.request(url).await?.text().await?)
    }

    pub async fn get_object_from_url(&self, url: &str) -> Result<XmlElement> {
        parse_xml(&self.fetch_filing(url).await?)
    }

    pub async fn extract_tables_from_filing_url(&self, url: &str) -> Result<Tables> {
        Ok(extract_tables(&self.fetch_filing(url).await?))
    }

    async fn json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        // Read bytes, then deserialize, retaining serde's structural error details.
        let body = self.request(url).await?.bytes().await?;
        Ok(serde_json::from_slice(&body)?)
    }

    async fn request(&self, url: &str) -> Result<reqwest::Response> {
        self.limiter.wait().await;
        let response = self.http.get(url).send().await?;
        if !response.status().is_success() {
            return Err(Error::Http {
                status: response.status().as_u16(),
                url: url.to_owned(),
            });
        }
        Ok(response)
    }
}

fn validate_document_url(value: &str) -> Result<()> {
    let url = Url::parse(value).map_err(|e| Error::InvalidInput(e.to_string()))?;
    if url.scheme() != "https"
        || !matches!(url.host_str(), Some("www.sec.gov" | "data.sec.gov"))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port_or_known_default() != Some(443)
    {
        return Err(Error::InvalidInput("document URL must use HTTPS on www.sec.gov or data.sec.gov without credentials or a custom port".into()));
    }
    Ok(())
}

#[cfg(test)]
#[path = "client_tests.rs"]
mod tests;
