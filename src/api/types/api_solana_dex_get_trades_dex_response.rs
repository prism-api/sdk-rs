pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetTradesDexResponse {
    #[serde(flatten)]
    pub paginated_response_fields: PaginatedResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SolanaDexTrade>>,
}

impl GetTradesDexResponse {
    pub fn builder() -> GetTradesDexResponseBuilder {
        <GetTradesDexResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTradesDexResponseBuilder {
    paginated_response_fields: Option<PaginatedResponse>,
    data: Option<Vec<SolanaDexTrade>>,
}

impl GetTradesDexResponseBuilder {
    pub fn paginated_response_fields(mut self, value: PaginatedResponse) -> Self {
        self.paginated_response_fields = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<SolanaDexTrade>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTradesDexResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paginated_response_fields`](GetTradesDexResponseBuilder::paginated_response_fields)
    pub fn build(self) -> Result<GetTradesDexResponse, BuildError> {
        Ok(GetTradesDexResponse {
            paginated_response_fields: self.paginated_response_fields.ok_or_else(|| BuildError::missing_field("paginated_response_fields"))?,
            data: self.data,
        })
    }
}
