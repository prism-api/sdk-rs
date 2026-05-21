pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayloadPagination {
    /// Maximum number of results to return in a single page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor returned by a previous response. Pass it to fetch the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PayloadPagination {
    pub fn builder() -> PayloadPaginationBuilder {
        <PayloadPaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayloadPaginationBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
}

impl PayloadPaginationBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayloadPagination`].
    pub fn build(self) -> Result<PayloadPagination, BuildError> {
        Ok(PayloadPagination {
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
