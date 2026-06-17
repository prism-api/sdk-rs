pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscribeSolanaDexTradesParams {
    /// The token addresses to filter by. Leave empty to subscribe to all tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_addresses: Option<Vec<String>>,
    /// The wallet addresses to filter by. Leave empty to subscribe to all wallets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexTradesParams {
    pub fn builder() -> SubscribeSolanaDexTradesParamsBuilder {
        <SubscribeSolanaDexTradesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexTradesParamsBuilder {
    token_addresses: Option<Vec<String>>,
    wallet_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexTradesParamsBuilder {
    pub fn token_addresses(mut self, value: Vec<String>) -> Self {
        self.token_addresses = Some(value);
        self
    }

    pub fn wallet_addresses(mut self, value: Vec<String>) -> Self {
        self.wallet_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexTradesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexTradesParams, BuildError> {
        Ok(SubscribeSolanaDexTradesParams {
            token_addresses: self.token_addresses,
            wallet_addresses: self.wallet_addresses,
        })
    }
}
