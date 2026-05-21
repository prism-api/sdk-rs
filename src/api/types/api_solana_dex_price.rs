pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPrice {
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

impl SolanaDexPrice {
    pub fn builder() -> SolanaDexPriceBuilder {
        <SolanaDexPriceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPriceBuilder {
    token_address: Option<String>,
    usd_price: Option<f64>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPriceBuilder {
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

    /// Consumes the builder and constructs a [`SolanaDexPrice`].
    pub fn build(self) -> Result<SolanaDexPrice, BuildError> {
        Ok(SolanaDexPrice {
            token_address: self.token_address,
            usd_price: self.usd_price,
            block_slot: self.block_slot,
            block_time: self.block_time,
        })
    }
}
