pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexWalletProfileIdentity2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_username: Option<String>,
}

impl SolanaDexWalletProfileIdentity2 {
    pub fn builder() -> SolanaDexWalletProfileIdentity2Builder {
        <SolanaDexWalletProfileIdentity2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfileIdentity2Builder {
    name: Option<String>,
    twitter_username: Option<String>,
    telegram_username: Option<String>,
}

impl SolanaDexWalletProfileIdentity2Builder {
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

    /// Consumes the builder and constructs a [`SolanaDexWalletProfileIdentity2`].
    pub fn build(self) -> Result<SolanaDexWalletProfileIdentity2, BuildError> {
        Ok(SolanaDexWalletProfileIdentity2 {
            name: self.name,
            twitter_username: self.twitter_username,
            telegram_username: self.telegram_username,
        })
    }
}
