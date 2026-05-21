pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPriceStatsDexRequest {
    /// Token addresses to retrieve price statistics for. Accepts between 1 and 1000 tokens per request.
    #[serde(default)]
    pub tokens: Vec<String>,
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
}

impl GetPriceStatsDexRequestBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPriceStatsDexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tokens`](GetPriceStatsDexRequestBuilder::tokens)
    pub fn build(self) -> Result<GetPriceStatsDexRequest, BuildError> {
        Ok(GetPriceStatsDexRequest {
            tokens: self.tokens.ok_or_else(|| BuildError::missing_field("tokens"))?,
        })
    }
}

