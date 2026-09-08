use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{fmt, str::FromStr};

/// Validated, zero-padded SEC central index key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cik(String);

impl Cik {
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim();
        if value.is_empty() || value.len() > 10 || !value.bytes().all(|c| c.is_ascii_digit()) {
            return Err(Error::InvalidInput(
                "CIK must contain 1 to 10 ASCII digits".into(),
            ));
        }
        Ok(Self(format!("{value:0>10}")))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn archive_component(&self) -> &str {
        let digits = self.0.trim_start_matches('0');
        if digits.is_empty() { "0" } else { digits }
    }
}

impl fmt::Display for Cik {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Cik {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl Serialize for Cik {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Cik {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct Visitor;
        impl de::Visitor<'_> for Visitor {
            type Value = Cik;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a CIK string or nonnegative integer of at most 10 digits")
            }
            fn visit_str<E: de::Error>(self, value: &str) -> std::result::Result<Cik, E> {
                Cik::new(value).map_err(E::custom)
            }
            fn visit_u64<E: de::Error>(self, value: u64) -> std::result::Result<Cik, E> {
                Cik::new(value.to_string()).map_err(E::custom)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}
