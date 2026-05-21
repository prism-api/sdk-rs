pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexWalletProfileMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexWalletProfileMetadata {
    pub fn builder() -> SolanaDexWalletProfileMetadataBuilder {
        <SolanaDexWalletProfileMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfileMetadataBuilder {
    last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexWalletProfileMetadataBuilder {
    pub fn last_trade_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_trade_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexWalletProfileMetadata`].
    pub fn build(self) -> Result<SolanaDexWalletProfileMetadata, BuildError> {
        Ok(SolanaDexWalletProfileMetadata {
            last_trade_at: self.last_trade_at,
        })
    }
}
