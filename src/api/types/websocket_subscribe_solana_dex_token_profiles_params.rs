pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexTokenProfilesParams {
    /// The token addresses to filter by. Leave empty to subscribe to all tokens.
    pub token_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexTokenProfilesParams {
    pub fn builder() -> SubscribeSolanaDexTokenProfilesParamsBuilder {
        <SubscribeSolanaDexTokenProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexTokenProfilesParamsBuilder {
    token_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexTokenProfilesParamsBuilder {
    pub fn token_addresses(mut self, value: Vec<String>) -> Self {
        self.token_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexTokenProfilesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexTokenProfilesParams, BuildError> {
        Ok(SubscribeSolanaDexTokenProfilesParams {
            token_addresses: self.token_addresses,
        })
    }
}
