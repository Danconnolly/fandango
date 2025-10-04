//! Integration tests for bitcoinsv-rpc library
//!
//! These tests require a running Bitcoin SV testnet node.
//!
//! ## Configuration
//!
//! Set the following environment variables:
//! - `BSV_NODE_URL`: URL of the Bitcoin SV node (default: http://localhost:18332)
//! - `BSV_NODE_USER`: RPC username (optional)
//! - `BSV_NODE_PASSWORD`: RPC password (optional)
//!
//! Example:
//! ```bash
//! export BSV_NODE_URL=http://localhost:18332
//! export BSV_NODE_USER=bitcoin
//! export BSV_NODE_PASSWORD=password
//! cargo test --test integration_tests -- --ignored
//! ```

use bitcoinsv_rpc::{NodeClient, SvNodeClient};

/// Helper to get node connection details from environment
fn get_node_config() -> (String, Option<String>, Option<String>) {
    let url =
        std::env::var("BSV_NODE_URL").unwrap_or_else(|_| "http://localhost:18332".to_string());
    let user = std::env::var("BSV_NODE_USER").ok();
    let password = std::env::var("BSV_NODE_PASSWORD").ok();
    (url, user, password)
}

/// Helper to create a client for testing
fn create_test_client() -> SvNodeClient {
    let (url, user, password) = get_node_config();
    SvNodeClient::new(&url, user, password).expect("Failed to create client")
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
async fn test_get_best_block_hash() {
    let client = create_test_client();

    let result = client.get_best_block_hash().await;
    assert!(
        result.is_ok(),
        "Failed to get best block hash: {:?}",
        result.err()
    );

    let hash = result.unwrap();
    let hash_str = hash.to_string();
    assert_eq!(hash_str.len(), 64, "Block hash should be 64 hex characters");
    assert!(
        hash_str.chars().all(|c| c.is_ascii_hexdigit()),
        "Block hash should only contain hex characters"
    );

    println!("Best block hash: {}", hash);
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
async fn test_get_block_header() {
    let client = create_test_client();

    // First get the best block hash
    let hash = client
        .get_best_block_hash()
        .await
        .expect("Failed to get best block hash");

    // Then get its header
    let result = client.get_block_header(&hash).await;
    assert!(
        result.is_ok(),
        "Failed to get block header: {:?}",
        result.err()
    );

    let header = result.unwrap();

    // Verify header properties
    let header_hash = header.hash();
    println!("Block header hash: {:?}", header_hash);
    println!("Block version: {}", header.version());
    println!("Block timestamp: {}", header.timestamp());
    println!("Block nonce: {}", header.nonce());
    println!("Block difficulty: {}", header.difficulty());

    // Basic sanity checks
    assert!(header.version() > 0, "Block version should be positive");
    assert!(header.timestamp() > 0, "Block timestamp should be positive");
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
async fn test_get_block() {
    let client = create_test_client();

    // First get the best block hash
    let hash = client
        .get_best_block_hash()
        .await
        .expect("Failed to get best block hash");

    // Then get the complete block
    let result = client.get_block(&hash).await;
    assert!(result.is_ok(), "Failed to get block: {:?}", result.err());

    let block = result.unwrap();

    // Verify block properties
    let header = block.header().expect("Failed to get block header");
    println!("Block header hash: {:?}", header.hash());
    println!("Number of transactions: {}", block.num_tx);

    // Basic sanity checks
    assert!(
        block.num_tx > 0,
        "Block should have at least one transaction (coinbase)"
    );
    assert!(header.version() > 0, "Block version should be positive");
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
async fn test_block_header_consistency() {
    let client = create_test_client();

    // Get the best block hash
    let hash = client
        .get_best_block_hash()
        .await
        .expect("Failed to get best block hash");

    // Get header directly
    let header = client
        .get_block_header(&hash)
        .await
        .expect("Failed to get block header");

    // Get block and extract header
    let block = client.get_block(&hash).await.expect("Failed to get block");
    let block_header = block
        .header()
        .expect("Failed to get block header from block");

    // The headers should be identical
    assert_eq!(
        format!("{:?}", header.hash()),
        format!("{:?}", block_header.hash()),
        "Header hash from get_block_header should match header hash from get_block"
    );
    assert_eq!(
        header.version(),
        block_header.version(),
        "Versions should match"
    );
    assert_eq!(
        header.timestamp(),
        block_header.timestamp(),
        "Timestamps should match"
    );
    assert_eq!(header.nonce(), block_header.nonce(), "Nonces should match");
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
async fn test_multiple_concurrent_requests() {
    let client = create_test_client();

    // Get the best block hash first
    let _hash = client
        .get_best_block_hash()
        .await
        .expect("Failed to get best block hash");

    // Clone the client for concurrent use
    let client2 = client.clone();
    let client3 = client.clone();

    // Make multiple concurrent requests using cloned clients
    let (result1, result2, result3) = tokio::join!(
        client.get_best_block_hash(),
        client2.get_best_block_hash(),
        client3.get_best_block_hash(),
    );

    // All should succeed
    assert!(
        result1.is_ok(),
        "Concurrent request 1 failed: {:?}",
        result1.err()
    );
    assert!(
        result2.is_ok(),
        "Concurrent request 2 failed: {:?}",
        result2.err()
    );
    assert!(
        result3.is_ok(),
        "Concurrent request 3 failed: {:?}",
        result3.err()
    );

    println!("All concurrent requests with cloned clients succeeded");
}

#[tokio::test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
async fn test_error_handling_invalid_hash() {
    use bitcoinsv::bitcoin::BlockHash;
    use hex::FromHex;

    let client = create_test_client();

    // Try to get a block with an invalid hash
    let invalid_hash_str = "0000000000000000000000000000000000000000000000000000000000000000";
    let invalid_hash = BlockHash::from_hex(invalid_hash_str).expect("Failed to parse hash");
    let result = client.get_block(&invalid_hash).await;

    // This should fail
    assert!(result.is_err(), "Getting invalid block should fail");
    println!("Expected error: {:?}", result.err());
}

#[tokio::test]
async fn test_client_creation() {
    // Test that we can create a client without connecting
    let client = SvNodeClient::new("http://localhost:18332", None, None);
    assert!(client.is_ok(), "Client creation should succeed");

    let client = SvNodeClient::new(
        "http://localhost:18332",
        Some("user".to_string()),
        Some("password".to_string()),
    );
    assert!(client.is_ok(), "Client creation with auth should succeed");
}

#[tokio::test]
async fn test_client_creation_invalid_url() {
    // Test that invalid URLs are rejected
    let client = SvNodeClient::new("not-a-url", None, None);
    assert!(
        client.is_err(),
        "Client creation with invalid URL should fail"
    );

    let client = SvNodeClient::new("localhost:8332", None, None);
    assert!(
        client.is_err(),
        "Client creation with URL missing scheme should fail"
    );
}
