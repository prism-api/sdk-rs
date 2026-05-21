pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPriceDexRequest {
    /// Token addresses to retrieve the latest prices for. Accepts between 1 and 1000 tokens per request.
    #[serde(default)]
    pub tokens: Vec<String>,
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
}

impl GetPriceDexRequestBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPriceDexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tokens`](GetPriceDexRequestBuilder::tokens)
    pub fn build(self) -> Result<GetPriceDexRequest, BuildError> {
        Ok(GetPriceDexRequest {
            tokens: self
                .tokens
                .ok_or_else(|| BuildError::missing_field("tokens"))?,
        })
    }
}
