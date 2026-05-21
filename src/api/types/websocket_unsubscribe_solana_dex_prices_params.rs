pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UnsubscribeSolanaDexPricesParams {
    pub tokens: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexPricesParams {
    pub fn builder() -> UnsubscribeSolanaDexPricesParamsBuilder {
        <UnsubscribeSolanaDexPricesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribeSolanaDexPricesParamsBuilder {
    tokens: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexPricesParamsBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribeSolanaDexPricesParams`].
    pub fn build(self) -> Result<UnsubscribeSolanaDexPricesParams, BuildError> {
        Ok(UnsubscribeSolanaDexPricesParams {
            tokens: self.tokens,
        })
    }
}
