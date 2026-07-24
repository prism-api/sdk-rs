pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchPositionProfilesDexRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SolanaDexProfileSearchPayloadFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SolanaDexProfileSearchPayloadSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_labels: Option<SolanaDexProfileSearchPayloadDynamicLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SolanaDexPositionProfilePayloadOptions>,
    /// Maximum number of results to return in a single page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor returned by a previous response. Pass it to fetch the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl SearchPositionProfilesDexRequest {
    pub fn builder() -> SearchPositionProfilesDexRequestBuilder {
        <SearchPositionProfilesDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchPositionProfilesDexRequestBuilder {
    filter: Option<SolanaDexProfileSearchPayloadFilter>,
    sort: Option<SolanaDexProfileSearchPayloadSort>,
    dynamic_labels: Option<SolanaDexProfileSearchPayloadDynamicLabels>,
    options: Option<SolanaDexPositionProfilePayloadOptions>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl SearchPositionProfilesDexRequestBuilder {
    pub fn filter(mut self, value: SolanaDexProfileSearchPayloadFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn sort(mut self, value: SolanaDexProfileSearchPayloadSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn dynamic_labels(mut self, value: SolanaDexProfileSearchPayloadDynamicLabels) -> Self {
        self.dynamic_labels = Some(value);
        self
    }

    pub fn options(mut self, value: SolanaDexPositionProfilePayloadOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchPositionProfilesDexRequest`].
    pub fn build(self) -> Result<SearchPositionProfilesDexRequest, BuildError> {
        Ok(SearchPositionProfilesDexRequest {
            filter: self.filter,
            sort: self.sort,
            dynamic_labels: self.dynamic_labels,
            options: self.options,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}

