pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscribeSolanaAssetsBalanceChangesParams {
    /// The owner addresses to filter by. Leave empty to subscribe to all owners.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_addresses: Option<Vec<String>>,
    /// The token addresses to filter by. Leave empty to subscribe to all tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaAssetsBalanceChangesParams {
    pub fn builder() -> SubscribeSolanaAssetsBalanceChangesParamsBuilder {
        <SubscribeSolanaAssetsBalanceChangesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaAssetsBalanceChangesParamsBuilder {
    owner_addresses: Option<Vec<String>>,
    token_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaAssetsBalanceChangesParamsBuilder {
    pub fn owner_addresses(mut self, value: Vec<String>) -> Self {
        self.owner_addresses = Some(value);
        self
    }

    pub fn token_addresses(mut self, value: Vec<String>) -> Self {
        self.token_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaAssetsBalanceChangesParams`].
    pub fn build(self) -> Result<SubscribeSolanaAssetsBalanceChangesParams, BuildError> {
        Ok(SubscribeSolanaAssetsBalanceChangesParams {
            owner_addresses: self.owner_addresses,
            token_addresses: self.token_addresses,
        })
    }
}
