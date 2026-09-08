/// Errors retain HTTP status and source errors so callers can handle failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InvalidInput(String),
    #[error("invalid SEC response: {0}")]
    InvalidResponse(String),
    #[error("HTTP {status} from {url}")]
    Http { status: u16, url: String },
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("invalid JSON response: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid XML: {0}")]
    Xml(#[from] roxmltree::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
