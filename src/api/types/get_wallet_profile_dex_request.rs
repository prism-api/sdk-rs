pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWalletProfileDexRequest {
    /// Wallet address to retrieve the profile for.
    #[serde(default)]
    pub wallet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SolanaDexWalletProfilePayloadOptions>,
}

impl GetWalletProfileDexRequest {
    pub fn builder() -> GetWalletProfileDexRequestBuilder {
        <GetWalletProfileDexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWalletProfileDexRequestBuilder {
    wallet: Option<String>,
    options: Option<SolanaDexWalletProfilePayloadOptions>,
}

impl GetWalletProfileDexRequestBuilder {
    pub fn wallet(mut self, value: impl Into<String>) -> Self {
        self.wallet = Some(value.into());
        self
    }

    pub fn options(mut self, value: SolanaDexWalletProfilePayloadOptions) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWalletProfileDexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`wallet`](GetWalletProfileDexRequestBuilder::wallet)
    pub fn build(self) -> Result<GetWalletProfileDexRequest, BuildError> {
        Ok(GetWalletProfileDexRequest {
            wallet: self
                .wallet
                .ok_or_else(|| BuildError::missing_field("wallet"))?,
            options: self.options,
        })
    }
}
