//! # Prism API v1 SDK
//!
//! The official Rust SDK for the Prism API v1.
//!
//! ## Getting Started
//!
//! ```rust
//! use prism_rs_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         api_key: Some("<value>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = Client::new(config).expect("Failed to build client");
//!     client
//!         .api
//!         .solana
//!         .dex
//!         .get_wallet_profile(
//!             &GetWalletProfileDexRequest {
//!                 wallet: "suqh5sHtr8HyJ7q8scBimULPkPpA557prMG47xCHQfK".to_string(),
//!                 options: Some(SolanaDexWalletProfilePayloadOptions {
//!                     include_metadata: Some(true),
//!                     include_labels: Some(true),
//!                     include_metrics: Some(vec![SolanaDexWalletProfileTimeWindowEnum::Window7D]),
//!                     ..Default::default()
//!                 }),
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod error;
pub mod core;
pub mod config;
pub mod client;
pub mod prelude;
pub mod environment;

pub use error::{ApiError, BuildError};
pub use environment::{*};
pub use api::{*};
pub use core::{*};
pub use config::{*};
pub use client::{*};

