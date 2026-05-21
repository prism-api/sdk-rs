pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UnsubscribeSolanaDexWalletProfilesParams {
    pub wallets: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexWalletProfilesParams {
    pub fn builder() -> UnsubscribeSolanaDexWalletProfilesParamsBuilder {
        <UnsubscribeSolanaDexWalletProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribeSolanaDexWalletProfilesParamsBuilder {
    wallets: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexWalletProfilesParamsBuilder {
    pub fn wallets(mut self, value: Vec<String>) -> Self {
        self.wallets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribeSolanaDexWalletProfilesParams`].
    pub fn build(self) -> Result<UnsubscribeSolanaDexWalletProfilesParams, BuildError> {
        Ok(UnsubscribeSolanaDexWalletProfilesParams {
            wallets: self.wallets,
        })
    }
}
