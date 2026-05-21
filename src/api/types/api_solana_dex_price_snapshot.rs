pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPriceSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume: Option<f64>,
}

impl SolanaDexPriceSnapshot {
    pub fn builder() -> SolanaDexPriceSnapshotBuilder {
        <SolanaDexPriceSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPriceSnapshotBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    usd_price: Option<f64>,
    usd_volume: Option<f64>,
}

impl SolanaDexPriceSnapshotBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn usd_price(mut self, value: f64) -> Self {
        self.usd_price = Some(value);
        self
    }

    pub fn usd_volume(mut self, value: f64) -> Self {
        self.usd_volume = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPriceSnapshot`].
    pub fn build(self) -> Result<SolanaDexPriceSnapshot, BuildError> {
        Ok(SolanaDexPriceSnapshot {
            timestamp: self.timestamp,
            usd_price: self.usd_price,
            usd_volume: self.usd_volume,
        })
    }
}
