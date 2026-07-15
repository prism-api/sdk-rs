pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPriceDexRequest {
    /// Token addresses to retrieve the latest prices for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<String>>,
    /// Pool addresses to retrieve the latest prices for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pools: Option<Vec<String>>,
}

impl GetPriceDexRequest {
    pub fn builder() -> GetPriceDexRequestBuilder {
        <GetPriceDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPriceDexRequestBuilder {
    tokens: Option<Vec<String>>,
    pools: Option<Vec<String>>,
}

impl GetPriceDexRequestBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    pub fn pools(mut self, value: Vec<String>) -> Self {
        self.pools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPriceDexRequest`].
    pub fn build(self) -> Result<GetPriceDexRequest, BuildError> {
        Ok(GetPriceDexRequest {
            tokens: self.tokens,
            pools: self.pools,
        })
    }
}

