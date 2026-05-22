pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexWalletProfile2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub synced_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<SolanaDexWalletProfileLabelEnum2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_labels: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SolanaDexWalletProfileMetadata2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<SolanaDexWalletProfileIdentity2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HashMap<String, SolanaDexWalletProfileMetrics2>>,
}

impl SolanaDexWalletProfile2 {
    pub fn builder() -> SolanaDexWalletProfile2Builder {
        <SolanaDexWalletProfile2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfile2Builder {
    updated_at: Option<DateTime<FixedOffset>>,
    synced_at: Option<DateTime<FixedOffset>>,
    wallet_address: Option<String>,
    labels: Option<Vec<SolanaDexWalletProfileLabelEnum2>>,
    matched_labels: Option<HashMap<String, Vec<String>>>,
    dynamic_labels: Option<Vec<String>>,
    metadata: Option<SolanaDexWalletProfileMetadata2>,
    identity: Option<SolanaDexWalletProfileIdentity2>,
    metrics: Option<HashMap<String, SolanaDexWalletProfileMetrics2>>,
}

impl SolanaDexWalletProfile2Builder {
    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn synced_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.synced_at = Some(value);
        self
    }

    pub fn wallet_address(mut self, value: impl Into<String>) -> Self {
        self.wallet_address = Some(value.into());
        self
    }

    pub fn labels(mut self, value: Vec<SolanaDexWalletProfileLabelEnum2>) -> Self {
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

    pub fn metadata(mut self, value: SolanaDexWalletProfileMetadata2) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn identity(mut self, value: SolanaDexWalletProfileIdentity2) -> Self {
        self.identity = Some(value);
        self
    }

    pub fn metrics(mut self, value: HashMap<String, SolanaDexWalletProfileMetrics2>) -> Self {
        self.metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexWalletProfile2`].
    pub fn build(self) -> Result<SolanaDexWalletProfile2, BuildError> {
        Ok(SolanaDexWalletProfile2 {
            updated_at: self.updated_at,
            synced_at: self.synced_at,
            wallet_address: self.wallet_address,
            labels: self.labels,
            matched_labels: self.matched_labels,
            dynamic_labels: self.dynamic_labels,
            metadata: self.metadata,
            identity: self.identity,
            metrics: self.metrics,
        })
    }
}
