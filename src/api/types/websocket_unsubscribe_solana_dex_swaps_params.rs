pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UnsubscribeSolanaDexSwapsParams {
    pub wallets: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexSwapsParams {
    pub fn builder() -> UnsubscribeSolanaDexSwapsParamsBuilder {
        <UnsubscribeSolanaDexSwapsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribeSolanaDexSwapsParamsBuilder {
    wallets: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexSwapsParamsBuilder {
    pub fn wallets(mut self, value: Vec<String>) -> Self {
        self.wallets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribeSolanaDexSwapsParams`].
    pub fn build(self) -> Result<UnsubscribeSolanaDexSwapsParams, BuildError> {
        Ok(UnsubscribeSolanaDexSwapsParams {
            wallets: self.wallets,
        })
    }
}
