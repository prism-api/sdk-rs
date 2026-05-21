pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPriceCandlesDexRequest {
    /// Token address to retrieve price candles for.
    #[serde(default)]
    pub token: String,
    /// Start of the candle range, as a date-time RFC3339 string.
    /// Must be combined with `to` to define a bounded range.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub from: Option<DateTime<FixedOffset>>,
    /// End of the candle range, as a date-time RFC3339 string. Defaults to the current time.
    /// Must be combined with either `from` (to define a bounded range) or `count` (to return the N most recent candles ending at `to`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub to: Option<DateTime<FixedOffset>>,
    /// Number of candles to return, ending at `to`.
    /// Must be combined with `to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Sampling interval between data points, in seconds.
    #[serde(default)]
    pub interval: i64,
}

impl GetPriceCandlesDexRequest {
    pub fn builder() -> GetPriceCandlesDexRequestBuilder {
        <GetPriceCandlesDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPriceCandlesDexRequestBuilder {
    token: Option<String>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    count: Option<i64>,
    interval: Option<i64>,
}

impl GetPriceCandlesDexRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPriceCandlesDexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](GetPriceCandlesDexRequestBuilder::token)
    /// - [`interval`](GetPriceCandlesDexRequestBuilder::interval)
    pub fn build(self) -> Result<GetPriceCandlesDexRequest, BuildError> {
        Ok(GetPriceCandlesDexRequest {
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            from: self.from,
            to: self.to,
            count: self.count,
            interval: self.interval.ok_or_else(|| BuildError::missing_field("interval"))?,
        })
    }
}

