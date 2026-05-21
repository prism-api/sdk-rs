pub use crate::prelude::*;

/// Controls which optional sections are included in each returned token profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexTokenProfilePayloadOptions {
    /// When true, includes the `metadata` object in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
    /// When true, includes the `market` object (price, liquidity, supply) in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_market: Option<bool>,
    /// When true, includes the `labels` array in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_labels: Option<bool>,
    /// Time windows for which metrics should be included. Windows not listed are omitted from the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metrics: Option<Vec<SolanaDexTokenProfileTimeWindowEnum>>,
}

impl SolanaDexTokenProfilePayloadOptions {
    pub fn builder() -> SolanaDexTokenProfilePayloadOptionsBuilder {
        <SolanaDexTokenProfilePayloadOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTokenProfilePayloadOptionsBuilder {
    include_metadata: Option<bool>,
    include_market: Option<bool>,
    include_labels: Option<bool>,
    include_metrics: Option<Vec<SolanaDexTokenProfileTimeWindowEnum>>,
}

impl SolanaDexTokenProfilePayloadOptionsBuilder {
    pub fn include_metadata(mut self, value: bool) -> Self {
        self.include_metadata = Some(value);
        self
    }

    pub fn include_market(mut self, value: bool) -> Self {
        self.include_market = Some(value);
        self
    }

    pub fn include_labels(mut self, value: bool) -> Self {
        self.include_labels = Some(value);
        self
    }

    pub fn include_metrics(mut self, value: Vec<SolanaDexTokenProfileTimeWindowEnum>) -> Self {
        self.include_metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexTokenProfilePayloadOptions`].
    pub fn build(self) -> Result<SolanaDexTokenProfilePayloadOptions, BuildError> {
        Ok(SolanaDexTokenProfilePayloadOptions {
            include_metadata: self.include_metadata,
            include_market: self.include_market,
            include_labels: self.include_labels,
            include_metrics: self.include_metrics,
        })
    }
}
