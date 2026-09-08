use crate::{Cik, Error, Filing, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanySubmissions {
    pub cik: Cik,
    pub name: String,
    pub tickers: Vec<String>,
    pub filings: Filings,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filings {
    pub recent: RecentFilings,
    pub files: Vec<HistoricalFile>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFile {
    pub name: String,
    pub filing_count: usize,
    pub filing_from: NaiveDate,
    pub filing_to: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentFilings {
    pub form: Vec<String>,
    pub primary_document: Vec<String>,
    pub filing_date: Vec<NaiveDate>,
    pub accession_number: Vec<String>,
    #[serde(rename = "isXBRL")]
    pub is_xbrl: Vec<u8>,
    pub act: Vec<String>,
    pub primary_doc_description: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl RecentFilings {
    /// Select metadata from `filings.recent`; does not traverse historical files.
    pub fn select(&self, cik: &Cik, after: NaiveDate, forms: &[&str]) -> Result<Vec<Filing>> {
        let count = self.form.len();
        let lengths = [
            self.primary_document.len(),
            self.filing_date.len(),
            self.accession_number.len(),
            self.is_xbrl.len(),
            self.act.len(),
            self.primary_doc_description.len(),
        ];
        if lengths.iter().any(|&length| length != count) {
            return Err(Error::InvalidResponse(
                "recent filing columns have different lengths".into(),
            ));
        }
        Ok((0..count)
            .filter(|&i| self.filing_date[i] > after && forms.contains(&self.form[i].as_str()))
            .map(|i| Filing {
                form: self.form[i].clone(),
                cik: cik.clone(),
                primary_document: self.primary_document[i].clone(),
                filing_date: self.filing_date[i],
                accession_number: self.accession_number[i].clone(),
                is_xbrl: self.is_xbrl[i],
                act: self.act[i].clone(),
                primary_doc_description: self.primary_doc_description[i].clone(),
            })
            .collect())
    }
}
