pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPositionProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub synced_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<SolanaDexPositionProfileLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_labels: Option<SolanaDexProfileDynamicLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SolanaDexPositionProfileMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HashMap<String, SolanaDexPositionProfileMetrics>>,
}

impl SolanaDexPositionProfile {
    pub fn builder() -> SolanaDexPositionProfileBuilder {
        <SolanaDexPositionProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPositionProfileBuilder {
    updated_at: Option<DateTime<FixedOffset>>,
    synced_at: Option<DateTime<FixedOffset>>,
    position_address: Option<String>,
    wallet_address: Option<String>,
    token_address: Option<String>,
    labels: Option<SolanaDexPositionProfileLabels>,
    dynamic_labels: Option<SolanaDexProfileDynamicLabels>,
    metadata: Option<SolanaDexPositionProfileMetadata>,
    metrics: Option<HashMap<String, SolanaDexPositionProfileMetrics>>,
}

impl SolanaDexPositionProfileBuilder {
    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn synced_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.synced_at = Some(value);
        self
    }

    pub fn position_address(mut self, value: impl Into<String>) -> Self {
        self.position_address = Some(value.into());
        self
    }

    pub fn wallet_address(mut self, value: impl Into<String>) -> Self {
        self.wallet_address = Some(value.into());
        self
    }

    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn labels(mut self, value: SolanaDexPositionProfileLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn dynamic_labels(mut self, value: SolanaDexProfileDynamicLabels) -> Self {
        self.dynamic_labels = Some(value);
        self
    }

    pub fn metadata(mut self, value: SolanaDexPositionProfileMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn metrics(mut self, value: HashMap<String, SolanaDexPositionProfileMetrics>) -> Self {
        self.metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPositionProfile`].
    pub fn build(self) -> Result<SolanaDexPositionProfile, BuildError> {
        Ok(SolanaDexPositionProfile {
            updated_at: self.updated_at,
            synced_at: self.synced_at,
            position_address: self.position_address,
            wallet_address: self.wallet_address,
            token_address: self.token_address,
            labels: self.labels,
            dynamic_labels: self.dynamic_labels,
            metadata: self.metadata,
            metrics: self.metrics,
        })
    }
}
