use crate::{Cik, Error, Result};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub(crate) struct TickerMapping {
    fields: Vec<String>,
    data: Vec<Vec<Value>>,
}

impl TickerMapping {
    pub(crate) fn lookup(self, ticker: &str) -> Result<Option<Cik>> {
        let field = |name| {
            self.fields
                .iter()
                .position(|value| value == name)
                .ok_or_else(|| Error::InvalidResponse(format!("ticker mapping is missing {name}")))
        };
        let ticker_index = field("ticker")?;
        let cik_index = field("cik")?;
        for row in self.data {
            let symbol = row
                .get(ticker_index)
                .and_then(Value::as_str)
                .ok_or_else(|| Error::InvalidResponse("ticker row has no ticker string".into()))?;
            if symbol.eq_ignore_ascii_case(ticker) {
                return Ok(Some(serde_json::from_value(
                    row.get(cik_index)
                        .cloned()
                        .ok_or_else(|| Error::InvalidResponse("ticker row has no CIK".into()))?,
                )?));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uses_field_names_and_handles_unknown_tickers() {
        let data = r#"{"fields":["ticker","cik"],"data":[["AAPL",320193]]}"#;
        let mapping: TickerMapping = serde_json::from_str(data).unwrap();
        assert_eq!(
            mapping.lookup("aapl").unwrap().unwrap().as_str(),
            "0000320193"
        );
        let mapping: TickerMapping = serde_json::from_str(data).unwrap();
        assert_eq!(mapping.lookup("UNKNOWN").unwrap(), None);
    }
    #[test]
    fn rejects_missing_schema_fields_and_invalid_rows() {
        for data in [
            r#"{"fields":["ticker"],"data":[]}"#,
            r#"{"fields":["ticker","cik"],"data":[[42,320193]]}"#,
            r#"{"fields":["ticker","cik"],"data":[["AAPL","bad"]]}"#,
        ] {
            let mapping: TickerMapping = serde_json::from_str(data).unwrap();
            assert!(mapping.lookup("AAPL").is_err());
        }
    }
}
