pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexWalletProfileIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_username: Option<String>,
}

impl SolanaDexWalletProfileIdentity {
    pub fn builder() -> SolanaDexWalletProfileIdentityBuilder {
        <SolanaDexWalletProfileIdentityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfileIdentityBuilder {
    name: Option<String>,
    twitter_username: Option<String>,
    telegram_username: Option<String>,
}

impl SolanaDexWalletProfileIdentityBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn twitter_username(mut self, value: impl Into<String>) -> Self {
        self.twitter_username = Some(value.into());
        self
    }

    pub fn telegram_username(mut self, value: impl Into<String>) -> Self {
        self.telegram_username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexWalletProfileIdentity`].
    pub fn build(self) -> Result<SolanaDexWalletProfileIdentity, BuildError> {
        Ok(SolanaDexWalletProfileIdentity {
            name: self.name,
            twitter_username: self.twitter_username,
            telegram_username: self.telegram_username,
        })
    }
}
