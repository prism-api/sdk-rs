pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexTokenProfileMetadata2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_trade_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram: Option<String>,
}

impl SolanaDexTokenProfileMetadata2 {
    pub fn builder() -> SolanaDexTokenProfileMetadata2Builder {
        <SolanaDexTokenProfileMetadata2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTokenProfileMetadata2Builder {
    last_trade_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    symbol: Option<String>,
    name: Option<String>,
    image: Option<String>,
    verified: Option<bool>,
    creator_address: Option<String>,
    twitter: Option<String>,
    discord: Option<String>,
    website: Option<String>,
    telegram: Option<String>,
}

impl SolanaDexTokenProfileMetadata2Builder {
    pub fn last_trade_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_trade_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn creator_address(mut self, value: impl Into<String>) -> Self {
        self.creator_address = Some(value.into());
        self
    }

    pub fn twitter(mut self, value: impl Into<String>) -> Self {
        self.twitter = Some(value.into());
        self
    }

    pub fn discord(mut self, value: impl Into<String>) -> Self {
        self.discord = Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    pub fn telegram(mut self, value: impl Into<String>) -> Self {
        self.telegram = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexTokenProfileMetadata2`].
    pub fn build(self) -> Result<SolanaDexTokenProfileMetadata2, BuildError> {
        Ok(SolanaDexTokenProfileMetadata2 {
            last_trade_at: self.last_trade_at,
            created_at: self.created_at,
            symbol: self.symbol,
            name: self.name,
            image: self.image,
            verified: self.verified,
            creator_address: self.creator_address,
            twitter: self.twitter,
            discord: self.discord,
            website: self.website,
            telegram: self.telegram,
        })
    }
}
