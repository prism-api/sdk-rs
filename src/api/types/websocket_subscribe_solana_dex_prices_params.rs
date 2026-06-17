pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexPricesParams {
    /// The token addresses to filter by. Leave empty to subscribe to all tokens.
    pub token_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexPricesParams {
    pub fn builder() -> SubscribeSolanaDexPricesParamsBuilder {
        <SubscribeSolanaDexPricesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexPricesParamsBuilder {
    token_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexPricesParamsBuilder {
    pub fn token_addresses(mut self, value: Vec<String>) -> Self {
        self.token_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexPricesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexPricesParams, BuildError> {
        Ok(SubscribeSolanaDexPricesParams {
            token_addresses: self.token_addresses,
        })
    }
}
