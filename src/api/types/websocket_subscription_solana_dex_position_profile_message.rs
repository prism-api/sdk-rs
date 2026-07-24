pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPositionProfileMessage {
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
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SolanaDexPositionProfileMetadata2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HashMap<String, SolanaDexPositionProfileMetrics2>>,
}

impl SolanaDexPositionProfileMessage {
    pub fn builder() -> SolanaDexPositionProfileMessageBuilder {
        <SolanaDexPositionProfileMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPositionProfileMessageBuilder {
    updated_at: Option<DateTime<FixedOffset>>,
    synced_at: Option<DateTime<FixedOffset>>,
    position_address: Option<String>,
    wallet_address: Option<String>,
    token_address: Option<String>,
    labels: Option<Vec<String>>,
    dynamic_labels: Option<Vec<String>>,
    metadata: Option<SolanaDexPositionProfileMetadata2>,
    metrics: Option<HashMap<String, SolanaDexPositionProfileMetrics2>>,
}

impl SolanaDexPositionProfileMessageBuilder {
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

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn dynamic_labels(mut self, value: Vec<String>) -> Self {
        self.dynamic_labels = Some(value);
        self
    }

    pub fn metadata(mut self, value: SolanaDexPositionProfileMetadata2) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn metrics(mut self, value: HashMap<String, SolanaDexPositionProfileMetrics2>) -> Self {
        self.metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPositionProfileMessage`].
    pub fn build(self) -> Result<SolanaDexPositionProfileMessage, BuildError> {
        Ok(SolanaDexPositionProfileMessage {
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
