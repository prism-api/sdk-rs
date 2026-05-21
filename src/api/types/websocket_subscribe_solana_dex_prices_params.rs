pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexPricesParams {
    pub tokens: Option<Vec<String>>,
}

impl SubscribeSolanaDexPricesParams {
    pub fn builder() -> SubscribeSolanaDexPricesParamsBuilder {
        <SubscribeSolanaDexPricesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexPricesParamsBuilder {
    tokens: Option<Vec<String>>,
}

impl SubscribeSolanaDexPricesParamsBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexPricesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexPricesParams, BuildError> {
        Ok(SubscribeSolanaDexPricesParams {
            tokens: self.tokens,
        })
    }
}
