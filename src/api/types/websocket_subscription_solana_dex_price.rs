pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPrice2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub block_time: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPrice2 {
    pub fn builder() -> SolanaDexPrice2Builder {
        <SolanaDexPrice2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPrice2Builder {
    token_address: Option<String>,
    usd_price: Option<f64>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPrice2Builder {
    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn usd_price(mut self, value: f64) -> Self {
        self.usd_price = Some(value);
        self
    }

    pub fn block_slot(mut self, value: i64) -> Self {
        self.block_slot = Some(value);
        self
    }

    pub fn block_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.block_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPrice2`].
    pub fn build(self) -> Result<SolanaDexPrice2, BuildError> {
        Ok(SolanaDexPrice2 {
            token_address: self.token_address,
            usd_price: self.usd_price,
            block_slot: self.block_slot,
            block_time: self.block_time,
        })
    }
}
