pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaginatedResponse {
    /// Total number of matching items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PaginatedResponse {
    pub fn builder() -> PaginatedResponseBuilder {
        <PaginatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResponseBuilder {
    count: Option<i64>,
    cursor: Option<String>,
}

impl PaginatedResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResponse`].
    pub fn build(self) -> Result<PaginatedResponse, BuildError> {
        Ok(PaginatedResponse {
            count: self.count,
            cursor: self.cursor,
        })
    }
}
