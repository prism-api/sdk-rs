pub use crate::prelude::*;

/// Controls which optional sections are included in each returned wallet profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexWalletProfilePayloadOptions {
    /// When true, includes the `metadata` object in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
    /// When true, includes the `identity` object in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_identity: Option<bool>,
    /// When true, includes the `labels` array in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_labels: Option<bool>,
    /// Time windows for which metrics should be included. Windows not listed are omitted from the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metrics: Option<Vec<SolanaDexWalletProfileTimeWindowEnum>>,
}

impl SolanaDexWalletProfilePayloadOptions {
    pub fn builder() -> SolanaDexWalletProfilePayloadOptionsBuilder {
        <SolanaDexWalletProfilePayloadOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfilePayloadOptionsBuilder {
    include_metadata: Option<bool>,
    include_identity: Option<bool>,
    include_labels: Option<bool>,
    include_metrics: Option<Vec<SolanaDexWalletProfileTimeWindowEnum>>,
}

impl SolanaDexWalletProfilePayloadOptionsBuilder {
    pub fn include_metadata(mut self, value: bool) -> Self {
        self.include_metadata = Some(value);
        self
    }

    pub fn include_identity(mut self, value: bool) -> Self {
        self.include_identity = Some(value);
        self
    }

    pub fn include_labels(mut self, value: bool) -> Self {
        self.include_labels = Some(value);
        self
    }

    pub fn include_metrics(mut self, value: Vec<SolanaDexWalletProfileTimeWindowEnum>) -> Self {
        self.include_metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexWalletProfilePayloadOptions`].
    pub fn build(self) -> Result<SolanaDexWalletProfilePayloadOptions, BuildError> {
        Ok(SolanaDexWalletProfilePayloadOptions {
            include_metadata: self.include_metadata,
            include_identity: self.include_identity,
            include_labels: self.include_labels,
            include_metrics: self.include_metrics,
        })
    }
}
