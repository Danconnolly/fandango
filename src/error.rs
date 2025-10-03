//! Error types for the Fandango library.

use thiserror::Error;

/// A specialized Result type for Fandango operations.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for Fandango operations.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON-RPC error response from node
    #[error("RPC error: code {code}, message: {message}")]
    Rpc { code: i32, message: String },

    /// Failed to parse JSON response
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    /// Failed to decode hex string
    #[error("Hex decoding failed: {0}")]
    Hex(#[from] hex::FromHexError),

    /// Failed to parse Bitcoin SV data
    #[error("Bitcoin SV parsing failed: {0}")]
    BitcoinSv(String),

    /// Invalid URL provided
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Authentication required but not provided
    #[error("Authentication required but credentials not provided")]
    AuthRequired,

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Other errors
    #[error("Error: {0}")]
    Other(String),
}
