pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchWalletProfilesDexResponse {
    #[serde(flatten)]
    pub paginated_response_fields: PaginatedResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SolanaDexWalletProfile>>,
}

impl SearchWalletProfilesDexResponse {
    pub fn builder() -> SearchWalletProfilesDexResponseBuilder {
        <SearchWalletProfilesDexResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchWalletProfilesDexResponseBuilder {
    paginated_response_fields: Option<PaginatedResponse>,
    data: Option<Vec<SolanaDexWalletProfile>>,
}

impl SearchWalletProfilesDexResponseBuilder {
    pub fn paginated_response_fields(mut self, value: PaginatedResponse) -> Self {
        self.paginated_response_fields = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<SolanaDexWalletProfile>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchWalletProfilesDexResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paginated_response_fields`](SearchWalletProfilesDexResponseBuilder::paginated_response_fields)
    pub fn build(self) -> Result<SearchWalletProfilesDexResponse, BuildError> {
        Ok(SearchWalletProfilesDexResponse {
            paginated_response_fields: self
                .paginated_response_fields
                .ok_or_else(|| BuildError::missing_field("paginated_response_fields"))?,
            data: self.data,
        })
    }
}
