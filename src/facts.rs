use crate::Cik;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::BTreeMap;

/// Standardized facts with taxonomy, units, filing dates and accession provenance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyFacts {
    pub cik: Cik,
    pub entity_name: String,
    pub facts: BTreeMap<String, BTreeMap<String, FactConcept>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactConcept {
    pub label: Option<String>,
    pub description: Option<String>,
    pub units: BTreeMap<String, Vec<XbrlFact>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FactValue {
    Number(Number),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XbrlFact {
    pub val: FactValue,
    pub accn: String,
    pub form: String,
    pub filed: NaiveDate,
    pub end: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fy: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
