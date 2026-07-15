pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscribeSolanaDexSwapsParams {
    /// The wallet addresses to filter by. Leave empty to subscribe to all wallets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_addresses: Option<Vec<String>>,
    /// The token addresses to filter by. Leave empty to subscribe to all tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_addresses: Option<Vec<String>>,
    /// The pool addresses to filter by. Leave empty to subscribe to all pools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexSwapsParams {
    pub fn builder() -> SubscribeSolanaDexSwapsParamsBuilder {
        <SubscribeSolanaDexSwapsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexSwapsParamsBuilder {
    wallet_addresses: Option<Vec<String>>,
    token_addresses: Option<Vec<String>>,
    pool_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexSwapsParamsBuilder {
    pub fn wallet_addresses(mut self, value: Vec<String>) -> Self {
        self.wallet_addresses = Some(value);
        self
    }

    pub fn token_addresses(mut self, value: Vec<String>) -> Self {
        self.token_addresses = Some(value);
        self
    }

    pub fn pool_addresses(mut self, value: Vec<String>) -> Self {
        self.pool_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexSwapsParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexSwapsParams, BuildError> {
        Ok(SubscribeSolanaDexSwapsParams {
            wallet_addresses: self.wallet_addresses,
            token_addresses: self.token_addresses,
            pool_addresses: self.pool_addresses,
        })
    }
}
