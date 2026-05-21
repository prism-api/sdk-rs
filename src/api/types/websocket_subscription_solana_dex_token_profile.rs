pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexTokenProfile2 {
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
    pub labels: Option<Vec<SolanaDexTokenProfileLabelEnum2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_labels: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SolanaDexTokenProfileMetadata2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<SolanaDexTokenProfileMarket2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HashMap<String, SolanaDexTokenProfileMetrics2>>,
}

impl SolanaDexTokenProfile2 {
    pub fn builder() -> SolanaDexTokenProfile2Builder {
        <SolanaDexTokenProfile2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTokenProfile2Builder {
    updated_at: Option<DateTime<FixedOffset>>,
    synced_at: Option<DateTime<FixedOffset>>,
    token_address: Option<String>,
    labels: Option<Vec<SolanaDexTokenProfileLabelEnum2>>,
    matched_labels: Option<HashMap<String, Vec<String>>>,
    dynamic_labels: Option<Vec<String>>,
    metadata: Option<SolanaDexTokenProfileMetadata2>,
    market: Option<SolanaDexTokenProfileMarket2>,
    metrics: Option<HashMap<String, SolanaDexTokenProfileMetrics2>>,
}

impl SolanaDexTokenProfile2Builder {
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

    pub fn labels(mut self, value: Vec<SolanaDexTokenProfileLabelEnum2>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn matched_labels(mut self, value: HashMap<String, Vec<String>>) -> Self {
        self.matched_labels = Some(value);
        self
    }

    pub fn dynamic_labels(mut self, value: Vec<String>) -> Self {
        self.dynamic_labels = Some(value);
        self
    }

    pub fn metadata(mut self, value: SolanaDexTokenProfileMetadata2) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn market(mut self, value: SolanaDexTokenProfileMarket2) -> Self {
        self.market = Some(value);
        self
    }

    pub fn metrics(mut self, value: HashMap<String, SolanaDexTokenProfileMetrics2>) -> Self {
        self.metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexTokenProfile2`].
    pub fn build(self) -> Result<SolanaDexTokenProfile2, BuildError> {
        Ok(SolanaDexTokenProfile2 {
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
