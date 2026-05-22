pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UnsubscribeSolanaDexPositionProfilesParams {
    pub positions: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexPositionProfilesParams {
    pub fn builder() -> UnsubscribeSolanaDexPositionProfilesParamsBuilder {
        <UnsubscribeSolanaDexPositionProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribeSolanaDexPositionProfilesParamsBuilder {
    positions: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexPositionProfilesParamsBuilder {
    pub fn positions(mut self, value: Vec<String>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribeSolanaDexPositionProfilesParams`].
    pub fn build(self) -> Result<UnsubscribeSolanaDexPositionProfilesParams, BuildError> {
        Ok(UnsubscribeSolanaDexPositionProfilesParams {
            positions: self.positions,
        })
    }
}
