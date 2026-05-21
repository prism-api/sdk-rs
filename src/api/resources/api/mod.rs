use crate::{ClientConfig, ApiError, HttpClient};
use crate::api::{*};

pub mod solana;
pub use solana::SolanaClient;
pub struct ApiClient {
    pub http_client: HttpClient,
    pub solana: SolanaClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
    http_client: HttpClient::new(config.clone())?,
    solana: SolanaClient::new(config.clone())?
})
    }

}

