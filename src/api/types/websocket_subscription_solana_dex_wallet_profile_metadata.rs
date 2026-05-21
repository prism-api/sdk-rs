pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexWalletProfileMetadata2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexWalletProfileMetadata2 {
    pub fn builder() -> SolanaDexWalletProfileMetadata2Builder {
        <SolanaDexWalletProfileMetadata2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfileMetadata2Builder {
    last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexWalletProfileMetadata2Builder {
    pub fn last_trade_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_trade_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexWalletProfileMetadata2`].
    pub fn build(self) -> Result<SolanaDexWalletProfileMetadata2, BuildError> {
        Ok(SolanaDexWalletProfileMetadata2 {
            last_trade_at: self.last_trade_at,
        })
    }
}
