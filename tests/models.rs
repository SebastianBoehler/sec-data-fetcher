use sec_data_fetcher::{Cik, CompanyFacts, FactValue, Filing, NaiveDate, RecentFilings};
use serde_json::json;

#[test]
fn validates_and_normalizes_ciks_in_json_and_strings() {
    assert_eq!(Cik::new(" 320193 ").unwrap().as_str(), "0000320193");
    assert_eq!(
        serde_json::from_str::<Cik>("320193").unwrap(),
        Cik::new("320193").unwrap()
    );
    assert_eq!(
        serde_json::to_string(&Cik::new("1").unwrap()).unwrap(),
        "\"0000000001\""
    );
    for invalid in ["", "-1", "1.0", "12345678901", "../123", "１２３"] {
        assert!(Cik::new(invalid).is_err());
    }
    for invalid in ["-1", "1.5", "true", "null"] {
        assert!(serde_json::from_str::<Cik>(invalid).is_err());
    }
}

fn recent() -> RecentFilings {
    serde_json::from_value(json!({
        "form": ["10-K", "8-K", "4"], "primaryDocument": ["annual.htm", "current.htm", "ownership.xml"],
        "filingDate": ["2026-01-02", "2026-01-01", "2026-01-03"],
        "accessionNumber": ["0000320193-26-000001", "0000320193-26-000002", "0000320193-26-000003"],
        "isXBRL": [1, 0, 0], "act": ["34", "34", "34"], "primaryDocDescription": ["Annual", "Current", "Ownership"]
    })).unwrap()
}

#[test]
fn selects_forms_with_a_strict_date_cutoff_and_rejects_misaligned_columns() {
    let cik = Cik::new("320193").unwrap();
    let after = "2026-01-01".parse::<NaiveDate>().unwrap();
    let reports = recent().select(&cik, after, &["10-K", "8-K"]).unwrap();
    assert_eq!(reports.len(), 1);
    assert_eq!(
        reports[0].url().unwrap(),
        "https://www.sec.gov/Archives/edgar/data/320193/000032019326000001/annual.htm"
    );
    let mut broken = recent();
    broken.act.pop();
    assert!(broken.select(&cik, after, &["10-K"]).is_err());
}

#[test]
fn validates_archive_paths_and_preserves_xsl_subdirectories() {
    let mut filing: Filing = recent()
        .select(
            &Cik::new("320193").unwrap(),
            "2026-01-01".parse().unwrap(),
            &["10-K"],
        )
        .unwrap()
        .remove(0);
    filing.primary_document = "xslF345X05/ownership.xml".into();
    assert!(filing.url().unwrap().ends_with("/xslF345X05/ownership.xml"));
    for document in ["", "../secret", "a/../b", "/absolute"] {
        filing.primary_document = document.into();
        assert!(filing.url().is_err());
    }
    filing.primary_document = "annual.htm".into();
    filing.accession_number = "../../evil".into();
    assert!(filing.url().is_err());
}

#[test]
fn preserves_fact_units_dates_and_integer_precision() {
    let facts: CompanyFacts = serde_json::from_value(json!({
        "cik": 320193, "entityName": "Apple Inc.", "facts": { "us-gaap": { "Assets": {
            "label": "Assets", "description": "Assets", "units": { "USD": [{
                "val": 9007199254740993_u64, "accn": "0000320193-26-000001", "form": "10-K", "filed": "2026-01-02", "end": "2025-12-31", "fp": "FY"
            }] }
        } } }
    })).unwrap();
    let fact = &facts.facts["us-gaap"]["Assets"].units["USD"][0];
    let FactValue::Number(number) = &fact.val else {
        panic!("numeric fact expected")
    };
    assert_eq!(number.as_u64(), Some(9007199254740993));
    assert_eq!(fact.end.to_string(), "2025-12-31");
}

#[test]
fn accepts_sec_concepts_with_null_labels_and_descriptions() {
    let concept: sec_data_fetcher::FactConcept =
        serde_json::from_str(r#"{"label":null,"description":null,"units":{"USD":[]}}"#).unwrap();
    assert!(concept.label.is_none());
    assert!(concept.description.is_none());
    assert!(concept.units.contains_key("USD"));
}
