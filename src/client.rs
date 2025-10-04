//! Bitcoin SV node client implementation.

use crate::error::Result;
use crate::rest::RestClient;
use crate::rpc::RpcClient;
use async_trait::async_trait;
use bitcoinsv::bitcoin::{Block, BlockHash, BlockHeader};

/// Trait for communicating with a Bitcoin node.
///
/// This trait defines the common interface for interacting with Bitcoin nodes,
/// allowing different client implementations (e.g., SV node, Teranode) to provide
/// the same functionality.
#[async_trait]
pub trait NodeClient {
    /// Returns the hash of the best (tip) block in the longest blockchain.
    async fn get_best_block_hash(&self) -> Result<BlockHash>;

    /// Returns the block header for the specified block hash.
    ///
    /// # Arguments
    ///
    /// * `block_hash` - The hash of the block to retrieve
    async fn get_block_header(&self, block_hash: &BlockHash) -> Result<BlockHeader>;

    /// Returns the complete block data for the specified block hash.
    ///
    /// # Arguments
    ///
    /// * `block_hash` - The hash of the block to retrieve
    async fn get_block(&self, block_hash: &BlockHash) -> Result<Block>;
}

/// Client for communicating with a Bitcoin SV node.
///
/// This client manages both JSON-RPC and REST API connections to a Bitcoin SV node.
///
/// # Example
///
/// ```no_run
/// use fandango::SvNodeClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SvNodeClient::new(
///         "http://localhost:8332",
///         Some("user".to_string()),
///         Some("password".to_string()),
///     )?;
///
///     let hash = client.get_best_block_hash().await?;
///     println!("Best block: {}", hash);
///
///     Ok(())
/// }
/// ```
pub struct SvNodeClient {
    rpc: RpcClient,
    rest: RestClient,
}

impl SvNodeClient {
    /// Creates a new client connection to a Bitcoin SV node.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL of the node (e.g., "http://localhost:8332")
    /// * `username` - Optional RPC username for authentication
    /// * `password` - Optional RPC password for authentication
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid.
    pub fn new(url: &str, username: Option<String>, password: Option<String>) -> Result<Self> {
        let rpc = RpcClient::new(url, username, password)?;
        let rest = RestClient::new(url)?;

        Ok(Self { rpc, rest })
    }

    /// Returns the hash of the best (tip) block in the longest blockchain.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use fandango::SvNodeClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SvNodeClient::new("http://localhost:8332", None, None)?;
    /// let hash = client.get_best_block_hash().await?;
    /// println!("Best block hash: {}", hash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_best_block_hash(&self) -> Result<BlockHash> {
        self.rpc.get_best_block_hash().await
    }

    /// Returns the block header for the specified block hash.
    ///
    /// # Arguments
    ///
    /// * `block_hash` - The hash of the block to retrieve
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use fandango::SvNodeClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SvNodeClient::new("http://localhost:8332", None, None)?;
    /// let hash = client.get_best_block_hash().await?;
    /// let header = client.get_block_header(&hash).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_block_header(&self, block_hash: &BlockHash) -> Result<BlockHeader> {
        self.rpc.get_block_header(block_hash).await
    }

    /// Returns the complete block data for the specified block hash.
    ///
    /// This method uses the REST API in binary mode for efficient data transfer.
    ///
    /// # Arguments
    ///
    /// * `block_hash` - The hash of the block to retrieve
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use fandango::SvNodeClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SvNodeClient::new("http://localhost:8332", None, None)?;
    /// let hash = client.get_best_block_hash().await?;
    /// let block = client.get_block(&hash).await?;
    /// println!("Block has {} transactions", block.num_tx);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_block(&self, block_hash: &BlockHash) -> Result<Block> {
        self.rest.get_block(block_hash).await
    }
}

#[async_trait]
impl NodeClient for SvNodeClient {
    async fn get_best_block_hash(&self) -> Result<BlockHash> {
        self.rpc.get_best_block_hash().await
    }

    async fn get_block_header(&self, block_hash: &BlockHash) -> Result<BlockHeader> {
        self.rpc.get_block_header(block_hash).await
    }

    async fn get_block(&self, block_hash: &BlockHash) -> Result<Block> {
        self.rest.get_block(block_hash).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = SvNodeClient::new(
            "http://localhost:8332",
            Some("user".to_string()),
            Some("password".to_string()),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_creation_no_auth() {
        let client = SvNodeClient::new("http://localhost:8332", None, None);
        assert!(client.is_ok());
    }
}
