pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTradesDexRequest {
    /// Wallet address to filter trades by. When combined with `token`, returns only trades for that wallet on that token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    /// Token address to filter trades by. When combined with `wallet`, returns only trades for that wallet on that token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Maximum number of results to return in a single page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor returned by a previous response. Pass it to fetch the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetTradesDexRequest {
    pub fn builder() -> GetTradesDexRequestBuilder {
        <GetTradesDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTradesDexRequestBuilder {
    wallet: Option<String>,
    token: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl GetTradesDexRequestBuilder {
    pub fn wallet(mut self, value: impl Into<String>) -> Self {
        self.wallet = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetTradesDexRequest`].
    pub fn build(self) -> Result<GetTradesDexRequest, BuildError> {
        Ok(GetTradesDexRequest {
            wallet: self.wallet,
            token: self.token,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
