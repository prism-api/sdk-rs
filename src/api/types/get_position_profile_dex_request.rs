pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPositionProfileDexRequest {
    /// Wallet address of the position to retrieve.
    #[serde(default)]
    pub wallet: String,
    /// Token address of the position to retrieve.
    #[serde(default)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SolanaDexPositionProfilePayloadOptions>,
}

impl GetPositionProfileDexRequest {
    pub fn builder() -> GetPositionProfileDexRequestBuilder {
        <GetPositionProfileDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPositionProfileDexRequestBuilder {
    wallet: Option<String>,
    token: Option<String>,
    options: Option<SolanaDexPositionProfilePayloadOptions>,
}

impl GetPositionProfileDexRequestBuilder {
    pub fn wallet(mut self, value: impl Into<String>) -> Self {
        self.wallet = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn options(mut self, value: SolanaDexPositionProfilePayloadOptions) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPositionProfileDexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`wallet`](GetPositionProfileDexRequestBuilder::wallet)
    /// - [`token`](GetPositionProfileDexRequestBuilder::token)
    pub fn build(self) -> Result<GetPositionProfileDexRequest, BuildError> {
        Ok(GetPositionProfileDexRequest {
            wallet: self.wallet.ok_or_else(|| BuildError::missing_field("wallet"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            options: self.options,
        })
    }
}

