pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWalletProfileDexRequest {
    /// Wallet address to retrieve the profile for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
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
    wallet_address: Option<String>,
    options: Option<SolanaDexWalletProfilePayloadOptions>,
}

impl GetWalletProfileDexRequestBuilder {
    pub fn wallet_address(mut self, value: impl Into<String>) -> Self {
        self.wallet_address = Some(value.into());
        self
    }

    pub fn options(mut self, value: SolanaDexWalletProfilePayloadOptions) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWalletProfileDexRequest`].
    pub fn build(self) -> Result<GetWalletProfileDexRequest, BuildError> {
        Ok(GetWalletProfileDexRequest {
            wallet_address: self.wallet_address,
            options: self.options,
        })
    }
}

