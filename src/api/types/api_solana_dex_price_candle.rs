pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPriceCandle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub high: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub close: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl SolanaDexPriceCandle {
    pub fn builder() -> SolanaDexPriceCandleBuilder {
        <SolanaDexPriceCandleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPriceCandleBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    volume: Option<f64>,
    count: Option<i64>,
}

impl SolanaDexPriceCandleBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn open(mut self, value: f64) -> Self {
        self.open = Some(value);
        self
    }

    pub fn high(mut self, value: f64) -> Self {
        self.high = Some(value);
        self
    }

    pub fn low(mut self, value: f64) -> Self {
        self.low = Some(value);
        self
    }

    pub fn close(mut self, value: f64) -> Self {
        self.close = Some(value);
        self
    }

    pub fn volume(mut self, value: f64) -> Self {
        self.volume = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPriceCandle`].
    pub fn build(self) -> Result<SolanaDexPriceCandle, BuildError> {
        Ok(SolanaDexPriceCandle {
            timestamp: self.timestamp,
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            volume: self.volume,
            count: self.count,
        })
    }
}
