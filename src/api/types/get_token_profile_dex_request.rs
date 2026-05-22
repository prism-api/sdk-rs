pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTokenProfileDexRequest {
    /// Token address to retrieve the profile for.
    #[serde(default)]
    pub token: String,
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
    token: Option<String>,
    options: Option<SolanaDexTokenProfilePayloadOptions>,
}

impl GetTokenProfileDexRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn options(mut self, value: SolanaDexTokenProfilePayloadOptions) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTokenProfileDexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](GetTokenProfileDexRequestBuilder::token)
    pub fn build(self) -> Result<GetTokenProfileDexRequest, BuildError> {
        Ok(GetTokenProfileDexRequest {
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            options: self.options,
        })
    }
}

