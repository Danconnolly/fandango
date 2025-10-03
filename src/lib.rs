//! # Fandango
//!
//! Async Rust library for Bitcoin SV node communication.
//!
//! This library provides a high-level interface for interacting with Bitcoin SV nodes
//! via their JSON-RPC API and REST interface.
//!
//! ## Features
//!
//! - Async/await support with Tokio
//! - JSON-RPC interface for node commands
//! - REST API interface for efficient block retrieval
//! - Type-safe integration with the bitcoinsv crate
//!
//! ## Example
//!
//! ```no_run
//! use fandango::SvNodeClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SvNodeClient::new(
//!         "http://localhost:8332",
//!         Some("user".to_string()),
//!         Some("password".to_string()),
//!     )?;
//!
//!     let best_block_hash = client.get_best_block_hash().await?;
//!     println!("Best block hash: {}", best_block_hash);
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod rest;
mod rpc;

pub use client::{NodeClient, SvNodeClient};
pub use error::{Error, Result};
