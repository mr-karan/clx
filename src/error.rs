use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClxError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unsupported backend: {0}")]
    UnsupportedBackend(String),

    #[error("No response from AI provider")]
    NoResponse,
}

pub type Result<T> = std::result::Result<T, ClxError>;
