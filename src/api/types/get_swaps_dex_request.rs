pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSwapsDexRequest {
    /// Wallet address to filter swaps by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    /// Token address to filter swaps by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Pool address to filter swaps by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    /// Maximum number of results to return in a single page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor returned by a previous response. Pass it to fetch the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSwapsDexRequest {
    pub fn builder() -> GetSwapsDexRequestBuilder {
        <GetSwapsDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSwapsDexRequestBuilder {
    wallet: Option<String>,
    token: Option<String>,
    pool: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl GetSwapsDexRequestBuilder {
    pub fn wallet(mut self, value: impl Into<String>) -> Self {
        self.wallet = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn pool(mut self, value: impl Into<String>) -> Self {
        self.pool = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSwapsDexRequest`].
    pub fn build(self) -> Result<GetSwapsDexRequest, BuildError> {
        Ok(GetSwapsDexRequest {
            wallet: self.wallet,
            token: self.token,
            pool: self.pool,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}

