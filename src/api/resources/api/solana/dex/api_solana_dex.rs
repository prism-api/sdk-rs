use crate::{ClientConfig, ApiError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::api::{*};

pub struct DexClient {
    pub http_client: HttpClient,
}

impl DexClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
    http_client: HttpClient::new(config.clone())?
})
    }

    /// Returns a wallet profile for a specific wallet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_wallet_profile(&self, request: &GetWalletProfileDexRequest, options: Option<RequestOptions>) -> Result<SolanaDexWalletProfile, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/profiles/wallets/get-profile",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Filter, query, and sort wallet profiles based on specified metrics and conditions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search_wallet_profiles(&self, request: &SearchWalletProfilesDexRequest, options: Option<RequestOptions>) -> Result<SearchWalletProfilesDexResponse, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/profiles/wallets/search-profiles",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Returns the profile for a specific token.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_token_profile(&self, request: &GetTokenProfileDexRequest, options: Option<RequestOptions>) -> Result<SolanaDexTokenProfile, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/profiles/tokens/get-profile",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Filter, query, and sort token profiles based on specified metrics and conditions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search_token_profiles(&self, request: &SearchTokenProfilesDexRequest, options: Option<RequestOptions>) -> Result<SearchTokenProfilesDexResponse, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/profiles/tokens/search-profiles",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Returns trades for a combination of wallet, token and/or pool.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_trades(&self, request: &GetTradesDexRequest, options: Option<RequestOptions>) -> Result<GetTradesDexResponse, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/trades/get-trades",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Returns swaps for a combination of wallet, token and/or pool.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_swaps(&self, request: &GetSwapsDexRequest, options: Option<RequestOptions>) -> Result<GetSwapsDexResponse, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/swaps/get-swaps",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Returns prices for one or more tokens or pools.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_price(&self, request: &GetPriceDexRequest, options: Option<RequestOptions>) -> Result<Vec<SolanaDexPrice>, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/prices/get-price",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Returns price stats for one or more tokens or pools.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_price_stats(&self, request: &GetPriceStatsDexRequest, options: Option<RequestOptions>) -> Result<Vec<SolanaDexPriceStats>, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/prices/get-price-stats",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Returns price candles for a specific token and/or pool.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_price_candles(&self, request: &GetPriceCandlesDexRequest, options: Option<RequestOptions>) -> Result<Vec<SolanaDexPriceCandle>, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/prices/get-price-candles",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

    /// Returns price history for one or more tokens or pools.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_price_history(&self, request: &GetPriceHistoryDexRequest, options: Option<RequestOptions>) -> Result<Vec<SolanaDexPriceHistory>, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "v1/solana/dex/prices/get-price-history",
            Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
            None,
            options,
        ).await
    }

}

