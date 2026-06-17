pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexWalletProfilesParams {
    /// The wallet addresses to filter by. Leave empty to subscribe to all wallets.
    pub wallet_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexWalletProfilesParams {
    pub fn builder() -> SubscribeSolanaDexWalletProfilesParamsBuilder {
        <SubscribeSolanaDexWalletProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexWalletProfilesParamsBuilder {
    wallet_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexWalletProfilesParamsBuilder {
    pub fn wallet_addresses(mut self, value: Vec<String>) -> Self {
        self.wallet_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexWalletProfilesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexWalletProfilesParams, BuildError> {
        Ok(SubscribeSolanaDexWalletProfilesParams {
            wallet_addresses: self.wallet_addresses,
        })
    }
}
