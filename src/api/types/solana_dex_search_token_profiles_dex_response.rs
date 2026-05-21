pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchTokenProfilesDexResponse {
    #[serde(flatten)]
    pub paginated_response_fields: PaginatedResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SolanaDexTokenProfile>>,
}

impl SearchTokenProfilesDexResponse {
    pub fn builder() -> SearchTokenProfilesDexResponseBuilder {
        <SearchTokenProfilesDexResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchTokenProfilesDexResponseBuilder {
    paginated_response_fields: Option<PaginatedResponse>,
    data: Option<Vec<SolanaDexTokenProfile>>,
}

impl SearchTokenProfilesDexResponseBuilder {
    pub fn paginated_response_fields(mut self, value: PaginatedResponse) -> Self {
        self.paginated_response_fields = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<SolanaDexTokenProfile>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchTokenProfilesDexResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paginated_response_fields`](SearchTokenProfilesDexResponseBuilder::paginated_response_fields)
    pub fn build(self) -> Result<SearchTokenProfilesDexResponse, BuildError> {
        Ok(SearchTokenProfilesDexResponse {
            paginated_response_fields: self
                .paginated_response_fields
                .ok_or_else(|| BuildError::missing_field("paginated_response_fields"))?,
            data: self.data,
        })
    }
}
