//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Api**
//! - **Websocket**

use crate::{ClientConfig, ApiError};

pub mod api;
pub mod websocket;
pub struct Client {
    pub config: ClientConfig,
    pub api: ApiClient,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            api: ApiClient::new(config.clone())?
        })
    }

}

pub use api::ApiClient;
pub use websocket::WebsocketClient;
