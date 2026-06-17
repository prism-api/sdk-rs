pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexPoolsParams {
    /// The pool addresses to filter by. Leave empty to subscribe to all pools.
    pub pool_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexPoolsParams {
    pub fn builder() -> SubscribeSolanaDexPoolsParamsBuilder {
        <SubscribeSolanaDexPoolsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexPoolsParamsBuilder {
    pool_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexPoolsParamsBuilder {
    pub fn pool_addresses(mut self, value: Vec<String>) -> Self {
        self.pool_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexPoolsParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexPoolsParams, BuildError> {
        Ok(SubscribeSolanaDexPoolsParams {
            pool_addresses: self.pool_addresses,
        })
    }
}
