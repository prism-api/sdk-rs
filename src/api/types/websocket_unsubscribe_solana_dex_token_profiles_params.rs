pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UnsubscribeSolanaDexTokenProfilesParams {
    pub tokens: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexTokenProfilesParams {
    pub fn builder() -> UnsubscribeSolanaDexTokenProfilesParamsBuilder {
        <UnsubscribeSolanaDexTokenProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribeSolanaDexTokenProfilesParamsBuilder {
    tokens: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexTokenProfilesParamsBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribeSolanaDexTokenProfilesParams`].
    pub fn build(self) -> Result<UnsubscribeSolanaDexTokenProfilesParams, BuildError> {
        Ok(UnsubscribeSolanaDexTokenProfilesParams {
            tokens: self.tokens,
        })
    }
}
