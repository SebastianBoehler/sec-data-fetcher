use crate::{Cik, Error, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Filing metadata. Bodies are downloaded explicitly with `SecClient::fetch_filing`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filing {
    pub form: String,
    pub cik: Cik,
    pub primary_document: String,
    pub filing_date: NaiveDate,
    pub accession_number: String,
    #[serde(rename = "isXBRL")]
    pub is_xbrl: u8,
    pub act: String,
    pub primary_doc_description: String,
}

impl Filing {
    pub fn url(&self) -> Result<String> {
        let accession = self.accession_number.as_bytes();
        if accession.len() != 20
            || accession[10] != b'-'
            || accession[13] != b'-'
            || !accession
                .iter()
                .enumerate()
                .all(|(i, c)| i == 10 || i == 13 || c.is_ascii_digit())
        {
            return Err(Error::InvalidResponse(
                "invalid filing accession number".into(),
            ));
        }
        if self
            .primary_document
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(Error::InvalidResponse(
                "invalid primary document path".into(),
            ));
        }
        let mut url = reqwest::Url::parse("https://www.sec.gov/Archives/edgar/data/")
            .expect("constant SEC URL");
        url.path_segments_mut()
            .expect("hierarchical URL")
            .pop_if_empty()
            .push(self.cik.archive_component())
            .push(&self.accession_number.replace('-', ""))
            .extend(self.primary_document.split('/'));
        Ok(url.into())
    }
}
