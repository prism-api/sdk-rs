pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexTokenProfilesParams {
    pub tokens: Option<Vec<String>>,
}

impl SubscribeSolanaDexTokenProfilesParams {
    pub fn builder() -> SubscribeSolanaDexTokenProfilesParamsBuilder {
        <SubscribeSolanaDexTokenProfilesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexTokenProfilesParamsBuilder {
    tokens: Option<Vec<String>>,
}

impl SubscribeSolanaDexTokenProfilesParamsBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexTokenProfilesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexTokenProfilesParams, BuildError> {
        Ok(SubscribeSolanaDexTokenProfilesParams {
            tokens: self.tokens,
        })
    }
}
