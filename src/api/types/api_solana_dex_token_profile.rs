pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexTokenProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub synced_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<SolanaDexTokenProfileLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_labels: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_labels: Option<SolanaDexProfileDynamicLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SolanaDexTokenProfileMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<SolanaDexTokenProfileMarket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HashMap<String, SolanaDexTokenProfileMetrics>>,
}

impl SolanaDexTokenProfile {
    pub fn builder() -> SolanaDexTokenProfileBuilder {
        <SolanaDexTokenProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTokenProfileBuilder {
    updated_at: Option<DateTime<FixedOffset>>,
    synced_at: Option<DateTime<FixedOffset>>,
    token_address: Option<String>,
    labels: Option<SolanaDexTokenProfileLabels>,
    matched_labels: Option<HashMap<String, Vec<String>>>,
    dynamic_labels: Option<SolanaDexProfileDynamicLabels>,
    metadata: Option<SolanaDexTokenProfileMetadata>,
    market: Option<SolanaDexTokenProfileMarket>,
    metrics: Option<HashMap<String, SolanaDexTokenProfileMetrics>>,
}

impl SolanaDexTokenProfileBuilder {
    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn synced_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.synced_at = Some(value);
        self
    }

    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn labels(mut self, value: SolanaDexTokenProfileLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn matched_labels(mut self, value: HashMap<String, Vec<String>>) -> Self {
        self.matched_labels = Some(value);
        self
    }

    pub fn dynamic_labels(mut self, value: SolanaDexProfileDynamicLabels) -> Self {
        self.dynamic_labels = Some(value);
        self
    }

    pub fn metadata(mut self, value: SolanaDexTokenProfileMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn market(mut self, value: SolanaDexTokenProfileMarket) -> Self {
        self.market = Some(value);
        self
    }

    pub fn metrics(mut self, value: HashMap<String, SolanaDexTokenProfileMetrics>) -> Self {
        self.metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexTokenProfile`].
    pub fn build(self) -> Result<SolanaDexTokenProfile, BuildError> {
        Ok(SolanaDexTokenProfile {
            updated_at: self.updated_at,
            synced_at: self.synced_at,
            token_address: self.token_address,
            labels: self.labels,
            matched_labels: self.matched_labels,
            dynamic_labels: self.dynamic_labels,
            metadata: self.metadata,
            market: self.market,
            metrics: self.metrics,
        })
    }
}
