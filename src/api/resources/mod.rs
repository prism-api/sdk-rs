//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Api**
//! - **Websocket**

use crate::{ClientConfig, ApiError};
use crate::api::websocket::{SubscriptionConnector};

pub mod api;
pub mod websocket;
pub struct Client {
    pub config: ClientConfig,
    pub api: ApiClient,
    pub subscription: SubscriptionConnector,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            api: ApiClient::new(config.clone())?,
            subscription: SubscriptionConnector::new(config.base_url.clone(), config.api_key.as_ref().map(|k| k.to_string()).or_else(|| config.token.as_ref().map(|t| format!("Bearer {}", t))))
        })
    }

}

pub use api::ApiClient;
pub use websocket::WebsocketClient;
