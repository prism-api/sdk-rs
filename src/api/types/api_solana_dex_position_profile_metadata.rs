pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexPositionProfileMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPositionProfileMetadata {
    pub fn builder() -> SolanaDexPositionProfileMetadataBuilder {
        <SolanaDexPositionProfileMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPositionProfileMetadataBuilder {
    last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPositionProfileMetadataBuilder {
    pub fn last_trade_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_trade_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPositionProfileMetadata`].
    pub fn build(self) -> Result<SolanaDexPositionProfileMetadata, BuildError> {
        Ok(SolanaDexPositionProfileMetadata {
            last_trade_at: self.last_trade_at,
        })
    }
}
