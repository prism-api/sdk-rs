pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPriceStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price: Option<f64>,
    #[serde(rename = "usd_price_change_5m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price_change5m: Option<f64>,
    #[serde(rename = "usd_price_change_1h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price_change1h: Option<f64>,
    #[serde(rename = "usd_price_change_6h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price_change6h: Option<f64>,
    #[serde(rename = "usd_price_change_12h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price_change12h: Option<f64>,
    #[serde(rename = "usd_price_change_1d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price_change1d: Option<f64>,
    #[serde(rename = "usd_price_change_7d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price_change7d: Option<f64>,
    #[serde(rename = "usd_price_change_30d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_price_change30d: Option<f64>,
    #[serde(rename = "usd_volume_5m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume5m: Option<f64>,
    #[serde(rename = "usd_volume_1h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume1h: Option<f64>,
    #[serde(rename = "usd_volume_6h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume6h: Option<f64>,
    #[serde(rename = "usd_volume_12h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume12h: Option<f64>,
    #[serde(rename = "usd_volume_1d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume1d: Option<f64>,
    #[serde(rename = "usd_volume_7d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume7d: Option<f64>,
    #[serde(rename = "usd_volume_30d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume30d: Option<f64>,
    #[serde(rename = "usd_volume_change_5m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume_change5m: Option<f64>,
    #[serde(rename = "usd_volume_change_1h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume_change1h: Option<f64>,
    #[serde(rename = "usd_volume_change_6h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume_change6h: Option<f64>,
    #[serde(rename = "usd_volume_change_12h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume_change12h: Option<f64>,
    #[serde(rename = "usd_volume_change_1d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume_change1d: Option<f64>,
    #[serde(rename = "usd_volume_change_7d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume_change7d: Option<f64>,
    #[serde(rename = "usd_volume_change_30d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_volume_change30d: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub block_time: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPriceStats {
    pub fn builder() -> SolanaDexPriceStatsBuilder {
        <SolanaDexPriceStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPriceStatsBuilder {
    token_address: Option<String>,
    pool_address: Option<String>,
    usd_price: Option<f64>,
    usd_price_change5m: Option<f64>,
    usd_price_change1h: Option<f64>,
    usd_price_change6h: Option<f64>,
    usd_price_change12h: Option<f64>,
    usd_price_change1d: Option<f64>,
    usd_price_change7d: Option<f64>,
    usd_price_change30d: Option<f64>,
    usd_volume5m: Option<f64>,
    usd_volume1h: Option<f64>,
    usd_volume6h: Option<f64>,
    usd_volume12h: Option<f64>,
    usd_volume1d: Option<f64>,
    usd_volume7d: Option<f64>,
    usd_volume30d: Option<f64>,
    usd_volume_change5m: Option<f64>,
    usd_volume_change1h: Option<f64>,
    usd_volume_change6h: Option<f64>,
    usd_volume_change12h: Option<f64>,
    usd_volume_change1d: Option<f64>,
    usd_volume_change7d: Option<f64>,
    usd_volume_change30d: Option<f64>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
}

impl SolanaDexPriceStatsBuilder {
    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn pool_address(mut self, value: impl Into<String>) -> Self {
        self.pool_address = Some(value.into());
        self
    }

    pub fn usd_price(mut self, value: f64) -> Self {
        self.usd_price = Some(value);
        self
    }

    pub fn usd_price_change5m(mut self, value: f64) -> Self {
        self.usd_price_change5m = Some(value);
        self
    }

    pub fn usd_price_change1h(mut self, value: f64) -> Self {
        self.usd_price_change1h = Some(value);
        self
    }

    pub fn usd_price_change6h(mut self, value: f64) -> Self {
        self.usd_price_change6h = Some(value);
        self
    }

    pub fn usd_price_change12h(mut self, value: f64) -> Self {
        self.usd_price_change12h = Some(value);
        self
    }

    pub fn usd_price_change1d(mut self, value: f64) -> Self {
        self.usd_price_change1d = Some(value);
        self
    }

    pub fn usd_price_change7d(mut self, value: f64) -> Self {
        self.usd_price_change7d = Some(value);
        self
    }

    pub fn usd_price_change30d(mut self, value: f64) -> Self {
        self.usd_price_change30d = Some(value);
        self
    }

    pub fn usd_volume5m(mut self, value: f64) -> Self {
        self.usd_volume5m = Some(value);
        self
    }

    pub fn usd_volume1h(mut self, value: f64) -> Self {
        self.usd_volume1h = Some(value);
        self
    }

    pub fn usd_volume6h(mut self, value: f64) -> Self {
        self.usd_volume6h = Some(value);
        self
    }

    pub fn usd_volume12h(mut self, value: f64) -> Self {
        self.usd_volume12h = Some(value);
        self
    }

    pub fn usd_volume1d(mut self, value: f64) -> Self {
        self.usd_volume1d = Some(value);
        self
    }

    pub fn usd_volume7d(mut self, value: f64) -> Self {
        self.usd_volume7d = Some(value);
        self
    }

    pub fn usd_volume30d(mut self, value: f64) -> Self {
        self.usd_volume30d = Some(value);
        self
    }

    pub fn usd_volume_change5m(mut self, value: f64) -> Self {
        self.usd_volume_change5m = Some(value);
        self
    }

    pub fn usd_volume_change1h(mut self, value: f64) -> Self {
        self.usd_volume_change1h = Some(value);
        self
    }

    pub fn usd_volume_change6h(mut self, value: f64) -> Self {
        self.usd_volume_change6h = Some(value);
        self
    }

    pub fn usd_volume_change12h(mut self, value: f64) -> Self {
        self.usd_volume_change12h = Some(value);
        self
    }

    pub fn usd_volume_change1d(mut self, value: f64) -> Self {
        self.usd_volume_change1d = Some(value);
        self
    }

    pub fn usd_volume_change7d(mut self, value: f64) -> Self {
        self.usd_volume_change7d = Some(value);
        self
    }

    pub fn usd_volume_change30d(mut self, value: f64) -> Self {
        self.usd_volume_change30d = Some(value);
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

    /// Consumes the builder and constructs a [`SolanaDexPriceStats`].
    pub fn build(self) -> Result<SolanaDexPriceStats, BuildError> {
        Ok(SolanaDexPriceStats {
            token_address: self.token_address,
            pool_address: self.pool_address,
            usd_price: self.usd_price,
            usd_price_change5m: self.usd_price_change5m,
            usd_price_change1h: self.usd_price_change1h,
            usd_price_change6h: self.usd_price_change6h,
            usd_price_change12h: self.usd_price_change12h,
            usd_price_change1d: self.usd_price_change1d,
            usd_price_change7d: self.usd_price_change7d,
            usd_price_change30d: self.usd_price_change30d,
            usd_volume5m: self.usd_volume5m,
            usd_volume1h: self.usd_volume1h,
            usd_volume6h: self.usd_volume6h,
            usd_volume12h: self.usd_volume12h,
            usd_volume1d: self.usd_volume1d,
            usd_volume7d: self.usd_volume7d,
            usd_volume30d: self.usd_volume30d,
            usd_volume_change5m: self.usd_volume_change5m,
            usd_volume_change1h: self.usd_volume_change1h,
            usd_volume_change6h: self.usd_volume_change6h,
            usd_volume_change12h: self.usd_volume_change12h,
            usd_volume_change1d: self.usd_volume_change1d,
            usd_volume_change7d: self.usd_volume_change7d,
            usd_volume_change30d: self.usd_volume_change30d,
            block_slot: self.block_slot,
            block_time: self.block_time,
        })
    }
}
