pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexSwapsParams {
    pub wallets: Option<Vec<String>>,
}

impl SubscribeSolanaDexSwapsParams {
    pub fn builder() -> SubscribeSolanaDexSwapsParamsBuilder {
        <SubscribeSolanaDexSwapsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexSwapsParamsBuilder {
    wallets: Option<Vec<String>>,
}

impl SubscribeSolanaDexSwapsParamsBuilder {
    pub fn wallets(mut self, value: Vec<String>) -> Self {
        self.wallets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexSwapsParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexSwapsParams, BuildError> {
        Ok(SubscribeSolanaDexSwapsParams {
            wallets: self.wallets,
        })
    }
}
