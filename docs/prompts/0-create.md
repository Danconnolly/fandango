Here's the updated prompt with the REST API specification for block fetching:
Create a Rust library for interfacing with Bitcoin SV nodes via their JSON-RPC API and REST interface.

## Project Overview
- **Purpose**: Async Rust library for Bitcoin SV node communication
- **Initial scope**: SV Node support (Teranode support planned for future)
- **Development approach**: Test-Driven Development (TDD)
- **Async runtime**: tokio
- **Dependencies**: bitcoinsv crate (https://docs.rs/bitcoinsv/latest/bitcoinsv/)

## Architecture Requirements

### Connection Management
Create a connection struct (e.g., `SvNodeClient` or `NodeConnection`) that:
- Manages the JSON-RPC connection to the node
- Manages the REST API connection to the node
- Handles authentication (credentials/tokens as needed for the RPC interface)
- Uses tokio for async operations
- Properly manages connection lifecycle and error handling

### Initial API Functions
Implement these async methods on the connection struct:

1. **`get_best_block_hash()`**
   - Returns the hash of the best (tip) block in the longest chain
   - Return type: Result with appropriate error handling
   - Uses JSON-RPC interface

2. **`get_block_header()`**
   - Takes a block hash as parameter
   - Returns the block header information
   - Should use types from the bitcoinsv crate where applicable
   - Uses JSON-RPC interface

3. **`get_block()`**
   - Takes a block hash as parameter
   - Returns the complete block data
   - Should use types from the bitcoinsv crate where applicable
   - **IMPORTANT**: Must use the REST API in binary mode (not JSON-RPC)
   - See `docs/external/REST-interface.md` for REST API documentation
   - The binary mode provides more efficient block retrieval

## REST API Usage

### Block Retrieval via REST
- When implementing `get_block()`, use the REST interface documented in `docs/external/REST-interface.md`
- Use binary mode for efficient data transfer
- Parse the binary response into the appropriate bitcoinsv types
- Handle HTTP errors and connection issues appropriately

## Testing Requirements

### Test-Driven Development
- Write tests FIRST before implementation
- Each function should have unit tests
- Use mock/fake responses for unit tests where appropriate

### Integration Tests
- Create a comprehensive integration test suite in `tests/` directory
- Tests must work against a real Bitcoin SV testnet node
- Tests should verify both JSON-RPC and REST API communication
- Document how to configure the test node connection (environment variables, config file, etc.)
- Include setup instructions in a README or test documentation
- Tests should verify actual RPC/REST communication and data parsing

## Code Quality Standards
- Follow Rust best practices and idioms
- Include proper error handling with custom error types
- Add documentation comments (///) for public APIs
- Include examples in doc comments
- Use `cargo clippy` guidelines
- Format code with `rustfmt`

## Project Structure
project-root/
├── Cargo.toml
├── README.md
├── docs/
│   └── external/
│       └── REST-interface.md
├── src/
│   ├── lib.rs
│   ├── client.rs (or connection.rs)
│   ├── rpc.rs (JSON-RPC methods)
│   ├── rest.rs (REST API methods)
│   ├── error.rs
│   └── ... (additional modules as needed)
└── tests/
└── integration_tests.rs

## Implementation Notes
- The Bitcoin SV node exposes both JSON-RPC (with authentication) and REST interfaces
- JSON-RPC is used for `get_best_block_hash()` and `get_block_header()`
- REST API in binary mode is used for `get_block()` for efficiency
- Consider connection pooling or reuse for efficiency
- Make the API ergonomic and idiomatic for Rust async code
- Design the library to be extensible for future additions
- The REST interface may require different HTTP client configuration than JSON-RPC

## Getting Started
1. Review `docs/external/REST-interface.md` to understand the REST API binary mode
2. Set up the basic project structure with Cargo
3. Add dependencies (tokio, bitcoinsv, serde_json, reqwest or hyper for HTTP, etc.)
4. Write the first test for `get_best_block_hash()`
5. Implement the code to make that test pass
6. Continue with TDD cycle for `get_block_header()` and then `get_block()`
7. Create integration tests for all functions

Please begin by setting up the project structure and implementing the connection management and `get_best_block_hash()` function using TDD methodology.

