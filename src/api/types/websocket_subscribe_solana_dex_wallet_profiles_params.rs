pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexWalletProfilesParams {
    pub wallets: Option<Vec<String>>,
}

impl SubscribeSolanaDexWalletProfilesParams {
    pub fn builder() -> SubscribeSolanaDexWalletProfilesParamsBuilder {
        <SubscribeSolanaDexWalletProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexWalletProfilesParamsBuilder {
    wallets: Option<Vec<String>>,
}

impl SubscribeSolanaDexWalletProfilesParamsBuilder {
    pub fn wallets(mut self, value: Vec<String>) -> Self {
        self.wallets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexWalletProfilesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexWalletProfilesParams, BuildError> {
        Ok(SubscribeSolanaDexWalletProfilesParams {
            wallets: self.wallets,
        })
    }
}
