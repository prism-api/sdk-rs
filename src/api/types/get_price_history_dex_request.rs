pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPriceHistoryDexRequest {
    /// Token addresses to retrieve price history for. Accepts between 1 and 100 tokens per request.
    #[serde(default)]
    pub tokens: Vec<String>,
    /// Start of the history range, as a date-time RFC3339 string.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub from: DateTime<FixedOffset>,
    /// End of the history range, as a date-time RFC3339 string. Defaults to the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub to: Option<DateTime<FixedOffset>>,
    /// Sampling interval between data points, in seconds.
    #[serde(default)]
    pub interval: i64,
}

impl GetPriceHistoryDexRequest {
    pub fn builder() -> GetPriceHistoryDexRequestBuilder {
        <GetPriceHistoryDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPriceHistoryDexRequestBuilder {
    tokens: Option<Vec<String>>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    interval: Option<i64>,
}

impl GetPriceHistoryDexRequestBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
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

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPriceHistoryDexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tokens`](GetPriceHistoryDexRequestBuilder::tokens)
    /// - [`from`](GetPriceHistoryDexRequestBuilder::from)
    /// - [`interval`](GetPriceHistoryDexRequestBuilder::interval)
    pub fn build(self) -> Result<GetPriceHistoryDexRequest, BuildError> {
        Ok(GetPriceHistoryDexRequest {
            tokens: self
                .tokens
                .ok_or_else(|| BuildError::missing_field("tokens"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to,
            interval: self
                .interval
                .ok_or_else(|| BuildError::missing_field("interval"))?,
        })
    }
}
