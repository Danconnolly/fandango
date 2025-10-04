//! JSON-RPC client implementation for Bitcoin SV nodes.

use crate::error::{Error, Result};
use bitcoinsv::bitcoin::{BlockHash, BlockHeader, Encodable};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC request structure
#[derive(Debug, Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<Value>,
}

/// JSON-RPC response structure
#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcError>,
    #[allow(dead_code)]
    id: String,
}

/// JSON-RPC error structure
#[derive(Debug, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

/// Client for JSON-RPC communication with Bitcoin SV node
pub(crate) struct RpcClient {
    url: String,
    client: Client,
    username: Option<String>,
    password: Option<String>,
}

impl RpcClient {
    /// Creates a new RPC client
    pub fn new(url: &str, username: Option<String>, password: Option<String>) -> Result<Self> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(Error::InvalidUrl(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        Ok(Self {
            url: url.to_string(),
            client: Client::new(),
            username,
            password,
        })
    }

    /// Makes an RPC call to the node
    async fn call<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T> {
        let request = RpcRequest {
            jsonrpc: "1.0".to_string(),
            id: "fandango".to_string(),
            method: method.to_string(),
            params,
        };

        let mut req = self.client.post(&self.url).json(&request);

        // Add basic auth if credentials provided
        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            req = req.basic_auth(username, Some(password));
        }

        let response = req.send().await?;
        let rpc_response: RpcResponse<T> = response.json().await?;

        if let Some(error) = rpc_response.error {
            return Err(Error::Rpc {
                code: error.code,
                message: error.message,
            });
        }

        rpc_response
            .result
            .ok_or_else(|| Error::Other("No result in RPC response".to_string()))
    }

    /// Gets the best block hash from the node
    pub async fn get_best_block_hash(&self) -> Result<BlockHash> {
        let hash_str: String = self.call("getbestblockhash", vec![]).await?;
        let mut bytes = hex::decode(&hash_str)?;
        bytes.reverse(); // Bitcoin hashes are in reverse byte order
        Ok(BlockHash::from_slice(&bytes))
    }

    /// Gets the block header for a given block hash
    pub async fn get_block_header(&self, block_hash: &BlockHash) -> Result<BlockHeader> {
        // Request verbose=false to get hex-encoded header
        let hex: String = self
            .call(
                "getblockheader",
                vec![Value::String(block_hash.to_string()), Value::Bool(false)],
            )
            .await?;

        // Decode hex to bytes
        let bytes = hex::decode(&hex)?;

        // Parse using bitcoinsv crate
        BlockHeader::from_binary(&mut &bytes[..])
            .map_err(|e| Error::BitcoinSv(format!("Failed to parse block header: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_client_creation() {
        let client = RpcClient::new(
            "http://localhost:8332",
            Some("user".to_string()),
            Some("pass".to_string()),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_rpc_client_invalid_url() {
        let client = RpcClient::new("localhost:8332", None, None);
        assert!(client.is_err());
    }

    // Integration tests will be in tests/integration_tests.rs
}
