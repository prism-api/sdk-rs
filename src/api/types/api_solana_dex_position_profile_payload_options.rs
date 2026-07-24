pub use crate::prelude::*;

/// Controls which optional sections are included in each returned position profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SolanaDexPositionProfilePayloadOptions {
    /// When true, includes the `metadata` object in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metadata: Option<bool>,
    /// When true, includes the `labels` array in each returned profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_labels: Option<bool>,
    /// Time windows for which metrics should be included. Windows not listed are omitted from the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_metrics: Option<Vec<SolanaDexPositionProfileTimeWindowEnum>>,
}

impl SolanaDexPositionProfilePayloadOptions {
    pub fn builder() -> SolanaDexPositionProfilePayloadOptionsBuilder {
        <SolanaDexPositionProfilePayloadOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPositionProfilePayloadOptionsBuilder {
    include_metadata: Option<bool>,
    include_labels: Option<bool>,
    include_metrics: Option<Vec<SolanaDexPositionProfileTimeWindowEnum>>,
}

impl SolanaDexPositionProfilePayloadOptionsBuilder {
    pub fn include_metadata(mut self, value: bool) -> Self {
        self.include_metadata = Some(value);
        self
    }

    pub fn include_labels(mut self, value: bool) -> Self {
        self.include_labels = Some(value);
        self
    }

    pub fn include_metrics(mut self, value: Vec<SolanaDexPositionProfileTimeWindowEnum>) -> Self {
        self.include_metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPositionProfilePayloadOptions`].
    pub fn build(self) -> Result<SolanaDexPositionProfilePayloadOptions, BuildError> {
        Ok(SolanaDexPositionProfilePayloadOptions {
            include_metadata: self.include_metadata,
            include_labels: self.include_labels,
            include_metrics: self.include_metrics,
        })
    }
}
