pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexPositionProfilesParams {
    /// The position addresses to filter by. Leave empty to subscribe to all positions.
    pub position_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexPositionProfilesParams {
    pub fn builder() -> SubscribeSolanaDexPositionProfilesParamsBuilder {
        <SubscribeSolanaDexPositionProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexPositionProfilesParamsBuilder {
    position_addresses: Option<Vec<String>>,
}

impl SubscribeSolanaDexPositionProfilesParamsBuilder {
    pub fn position_addresses(mut self, value: Vec<String>) -> Self {
        self.position_addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexPositionProfilesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexPositionProfilesParams, BuildError> {
        Ok(SubscribeSolanaDexPositionProfilesParams {
            position_addresses: self.position_addresses,
        })
    }
}
