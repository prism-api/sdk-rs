pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSwapsDexResponse {
    #[serde(flatten)]
    pub paginated_response_fields: PaginatedResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SolanaDexSwap>>,
}

impl GetSwapsDexResponse {
    pub fn builder() -> GetSwapsDexResponseBuilder {
        <GetSwapsDexResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSwapsDexResponseBuilder {
    paginated_response_fields: Option<PaginatedResponse>,
    data: Option<Vec<SolanaDexSwap>>,
}

impl GetSwapsDexResponseBuilder {
    pub fn paginated_response_fields(mut self, value: PaginatedResponse) -> Self {
        self.paginated_response_fields = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<SolanaDexSwap>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSwapsDexResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paginated_response_fields`](GetSwapsDexResponseBuilder::paginated_response_fields)
    pub fn build(self) -> Result<GetSwapsDexResponse, BuildError> {
        Ok(GetSwapsDexResponse {
            paginated_response_fields: self.paginated_response_fields.ok_or_else(|| BuildError::missing_field("paginated_response_fields"))?,
            data: self.data,
        })
    }
}
