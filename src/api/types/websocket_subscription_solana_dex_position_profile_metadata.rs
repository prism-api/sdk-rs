pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexPositionProfileMetadata2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPositionProfileMetadata2 {
    pub fn builder() -> SolanaDexPositionProfileMetadata2Builder {
        <SolanaDexPositionProfileMetadata2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPositionProfileMetadata2Builder {
    last_trade_at: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPositionProfileMetadata2Builder {
    pub fn last_trade_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_trade_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPositionProfileMetadata2`].
    pub fn build(self) -> Result<SolanaDexPositionProfileMetadata2, BuildError> {
        Ok(SolanaDexPositionProfileMetadata2 {
            last_trade_at: self.last_trade_at,
        })
    }
}
