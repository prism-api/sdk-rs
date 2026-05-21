//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Solana**

use crate::{ApiError, ClientConfig};

pub mod solana;
pub struct Client {
    pub config: ClientConfig,
    pub solana: SolanaClient,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            solana: SolanaClient::new(config.clone())?,
        })
    }
}

pub use solana::SolanaClient;
