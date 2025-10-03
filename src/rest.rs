//! REST API client implementation for Bitcoin SV nodes.

use crate::error::{Error, Result};
use bitcoinsv::bitcoin::Block;
use reqwest::Client;

/// Client for REST API communication with Bitcoin SV node
pub(crate) struct RestClient {
    base_url: String,
    client: Client,
}

impl RestClient {
    /// Creates a new REST client
    pub fn new(url: &str) -> Result<Self> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(Error::InvalidUrl(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        // Remove trailing slash if present
        let base_url = url.trim_end_matches('/').to_string();

        Ok(Self {
            base_url,
            client: Client::new(),
        })
    }

    /// Gets a block in binary format from the REST API
    ///
    /// Uses the endpoint: GET /rest/block/<BLOCK-HASH>.bin
    pub async fn get_block(&self, block_hash: &str) -> Result<Block> {
        let url = format!("{}/rest/block/{}.bin", self.base_url, block_hash);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(Error::Other(format!(
                "REST API returned error: {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;

        // Parse binary data using bitcoinsv crate
        // reqwest::Bytes is compatible with bitcoinsv's expected Bytes type
        Block::new(bytes).map_err(|e| Error::BitcoinSv(format!("Failed to parse block: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rest_client_creation() {
        let client = RestClient::new("http://localhost:8332");
        assert!(client.is_ok());
    }

    #[test]
    fn test_rest_client_invalid_url() {
        let client = RestClient::new("localhost:8332");
        assert!(client.is_err());
    }

    #[test]
    fn test_rest_client_removes_trailing_slash() {
        let client = RestClient::new("http://localhost:8332/").unwrap();
        assert_eq!(client.base_url, "http://localhost:8332");
    }

    // Integration tests will be in tests/integration_tests.rs
}
