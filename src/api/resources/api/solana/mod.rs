use crate::{ClientConfig, ApiError, HttpClient};
use crate::api::{*};

pub mod dex;
pub use dex::DexClient;
pub struct SolanaClient {
    pub http_client: HttpClient,
    pub dex: DexClient,
}

impl SolanaClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
    http_client: HttpClient::new(config.clone())?,
    dex: DexClient::new(config.clone())?
})
    }

}

