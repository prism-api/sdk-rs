pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPriceStatsDexRequest {
    /// Token addresses to retrieve price statistics for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<String>>,
    /// Pool addresses to retrieve price statistics for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pools: Option<Vec<String>>,
}

impl GetPriceStatsDexRequest {
    pub fn builder() -> GetPriceStatsDexRequestBuilder {
        <GetPriceStatsDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPriceStatsDexRequestBuilder {
    tokens: Option<Vec<String>>,
    pools: Option<Vec<String>>,
}

impl GetPriceStatsDexRequestBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    pub fn pools(mut self, value: Vec<String>) -> Self {
        self.pools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPriceStatsDexRequest`].
    pub fn build(self) -> Result<GetPriceStatsDexRequest, BuildError> {
        Ok(GetPriceStatsDexRequest {
            tokens: self.tokens,
            pools: self.pools,
        })
    }
}

