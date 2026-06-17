pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTokenProfileDexRequest {
    /// Token address to retrieve the profile for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SolanaDexTokenProfilePayloadOptions>,
}

impl GetTokenProfileDexRequest {
    pub fn builder() -> GetTokenProfileDexRequestBuilder {
        <GetTokenProfileDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTokenProfileDexRequestBuilder {
    token_address: Option<String>,
    options: Option<SolanaDexTokenProfilePayloadOptions>,
}

impl GetTokenProfileDexRequestBuilder {
    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn options(mut self, value: SolanaDexTokenProfilePayloadOptions) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTokenProfileDexRequest`].
    pub fn build(self) -> Result<GetTokenProfileDexRequest, BuildError> {
        Ok(GetTokenProfileDexRequest {
            token_address: self.token_address,
            options: self.options,
        })
    }
}

