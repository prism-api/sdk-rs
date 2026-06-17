pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscribeSolanaAssetsTransfersParams {
    /// The from addresses to filter by. Leave empty to subscribe to all from addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_addresses: Option<Vec<String>>,
    /// The to addresses to filter by. Leave empty to subscribe to all to addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaAssetsTransfersParams {
    pub fn builder() -> SubscribeSolanaAssetsTransfersParamsBuilder {
        <SubscribeSolanaAssetsTransfersParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaAssetsTransfersParamsBuilder {
    from_addresses: Option<Vec<String>>,
    to_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaAssetsTransfersParamsBuilder {
    pub fn from_addresses(mut self, value: Vec<String>) -> Self {
        self.from_addresses = Some(value);
        self
    }

    pub fn to_addresses(mut self, value: Vec<String>) -> Self {
        self.to_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaAssetsTransfersParams`].
    pub fn build(self) -> Result<SubscribeSolanaAssetsTransfersParams, BuildError> {
        Ok(SubscribeSolanaAssetsTransfersParams {
            from_addresses: self.from_addresses,
            to_addresses: self.to_addresses,
        })
    }
}
