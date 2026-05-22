pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexPositionProfilesParams {
    pub positions: Option<Vec<String>>,
}

impl SubscribeSolanaDexPositionProfilesParams {
    pub fn builder() -> SubscribeSolanaDexPositionProfilesParamsBuilder {
        <SubscribeSolanaDexPositionProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexPositionProfilesParamsBuilder {
    positions: Option<Vec<String>>,
}

impl SubscribeSolanaDexPositionProfilesParamsBuilder {
    pub fn positions(mut self, value: Vec<String>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexPositionProfilesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexPositionProfilesParams, BuildError> {
        Ok(SubscribeSolanaDexPositionProfilesParams {
            positions: self.positions,
        })
    }
}
