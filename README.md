# Fandango

[![Crates.io](https://img.shields.io/crates/v/fandango.svg)](https://crates.io/crates/fandango)
[![Documentation](https://docs.rs/fandango/badge.svg)](https://docs.rs/fandango)
[![License](https://img.shields.io/crates/l/fandango.svg)](./LICENSE)

An async Rust library for interfacing with Bitcoin SV nodes via their JSON-RPC API and REST interface.

## Features

- 🚀 **Async/await** support with Tokio
- 🔐 **JSON-RPC** interface for node commands with authentication support
- ⚡ **REST API** interface for efficient binary block retrieval
- 🔧 **Type-safe** integration with the [bitcoinsv](https://crates.io/crates/bitcoinsv) crate
- ✅ **Tested** with comprehensive unit and integration tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fandango = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use fandango::SvNodeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client connection to a Bitcoin SV node
    let client = SvNodeClient::new(
        "http://localhost:8332",
        Some("your_rpc_user".to_string()),
        Some("your_rpc_password".to_string()),
    )?;

    // Get the best block hash
    let best_hash = client.get_best_block_hash().await?;
    println!("Best block hash: {}", best_hash);

    // Get the block header
    let header = client.get_block_header(&best_hash).await?;
    println!("Block version: {}", header.version());
    println!("Block timestamp: {}", header.timestamp());

    // Get the complete block (uses REST API for efficiency)
    let block = client.get_block(&best_hash).await?;
    println!("Number of transactions: {}", block.num_tx);

    Ok(())
}
```

## API Methods

### `get_best_block_hash()`

Returns the hash of the best (tip) block in the longest blockchain.

**Returns:** `Result<bitcoinsv::bitcoin::BlockHash>`

**Example:**
```rust
let hash = client.get_best_block_hash().await?;
println!("Best block: {}", hash);
```

### `get_block_header(block_hash: &BlockHash)`

Returns the block header for a specified block hash. Uses the JSON-RPC interface.

**Parameters:**
- `block_hash`: The hash of the block to retrieve

**Returns:** `Result<bitcoinsv::bitcoin::BlockHeader>`

**Example:**
```rust
let hash = client.get_best_block_hash().await?;
let header = client.get_block_header(&hash).await?;
println!("Block difficulty: {}", header.difficulty());
```

### `get_block(block_hash: &BlockHash)`

Returns the complete block data for a specified block hash. Uses the REST API in binary mode for efficient data transfer.

**Parameters:**
- `block_hash`: The hash of the block to retrieve

**Returns:** `Result<bitcoinsv::bitcoin::Block>`

**Example:**
```rust
let hash = client.get_best_block_hash().await?;
let block = client.get_block(&hash).await?;
for tx in block.tx_iter() {
    // Process transactions
}
```

## Configuration

### Node Connection

The client requires a node URL and optionally authentication credentials:

```rust
// Without authentication (for nodes with rpcauth disabled)
let client = SvNodeClient::new("http://localhost:8332", None, None)?;

// With authentication
let client = SvNodeClient::new(
    "http://localhost:8332",
    Some("username".to_string()),
    Some("password".to_string()),
)?;

// For testnet (default port 18332)
let client = SvNodeClient::new(
    "http://localhost:18332",
    Some("username".to_string()),
    Some("password".to_string()),
)?;
```

### Bitcoin SV Node Setup

To use this library, you need a running Bitcoin SV node with:

1. **JSON-RPC enabled** (enabled by default)
2. **REST interface enabled** - Add `-rest` to your node configuration or command line
3. **Authentication configured** (if required) - Set `rpcuser` and `rpcpassword` in your bitcoin.conf

Example `bitcoin.conf`:
```ini
# Enable RPC server
server=1

# RPC credentials
rpcuser=your_username
rpcpassword=your_password

# RPC bind address (default: 127.0.0.1)
rpcbind=127.0.0.1

# RPC port (8332 for mainnet, 18332 for testnet)
rpcport=8332

# Enable REST API
rest=1
```

## Testing

### Unit Tests

Run the unit tests:

```bash
cargo test
```

### Integration Tests

Integration tests require a running Bitcoin SV node. Configure the connection using environment variables:

```bash
# Set node connection details
export BSV_NODE_URL=http://localhost:18332
export BSV_NODE_USER=your_username
export BSV_NODE_PASSWORD=your_password

# Run integration tests
cargo test --test integration -- --ignored
```

**Environment Variables:**
- `BSV_NODE_URL`: URL of the Bitcoin SV node (default: `http://localhost:18332`)
- `BSV_NODE_USER`: RPC username (optional, omit if authentication is disabled)
- `BSV_NODE_PASSWORD`: RPC password (optional, omit if authentication is disabled)

**Note:** Integration tests are marked with `#[ignore]` by default and must be explicitly run with the `--ignored` flag.

## Error Handling

The library provides detailed error types via the `Error` enum:

```rust
use fandango::{SvNodeClient, Error};

let hash = client.get_best_block_hash().await?;
match client.get_block(&hash).await {
    Ok(block) => println!("Got block with {} transactions", block.num_tx),
    Err(Error::Rpc { code, message }) => {
        eprintln!("RPC error {}: {}", code, message);
    }
    Err(Error::Http(e)) => {
        eprintln!("HTTP error: {}", e);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Architecture

The library is structured into several modules:

- **`client`**: Main `SvNodeClient` struct providing the public API
- **`rpc`**: JSON-RPC client implementation for RPC methods
- **`rest`**: REST API client for efficient binary block retrieval
- **`error`**: Error types and Result type alias

### Why REST for Blocks?

The library uses the REST API (instead of JSON-RPC) for retrieving complete blocks because:

1. **Binary format** is more efficient than JSON for large data structures
2. **Reduced overhead** - no JSON encoding/decoding for block data
3. **Better performance** for large blocks (Bitcoin SV supports very large blocks)

## Development Status

This library is under active development. Current implementation includes:

- ✅ Connection management (JSON-RPC and REST)
- ✅ `get_best_block_hash()`
- ✅ `get_block_header()`
- ✅ `get_block()` via REST API
- 🚧 Additional RPC methods (planned)
- 🚧 Teranode support (planned)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with the [bitcoinsv](https://crates.io/crates/bitcoinsv) crate for Bitcoin SV types
- Uses [tokio](https://tokio.rs/) for async runtime
- HTTP client powered by [reqwest](https://docs.rs/reqwest/)