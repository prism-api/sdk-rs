pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UnsubscribeSolanaDexTradesParams {
    pub tokens: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexTradesParams {
    pub fn builder() -> UnsubscribeSolanaDexTradesParamsBuilder {
        <UnsubscribeSolanaDexTradesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnsubscribeSolanaDexTradesParamsBuilder {
    tokens: Option<Vec<String>>,
}

impl UnsubscribeSolanaDexTradesParamsBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnsubscribeSolanaDexTradesParams`].
    pub fn build(self) -> Result<UnsubscribeSolanaDexTradesParams, BuildError> {
        Ok(UnsubscribeSolanaDexTradesParams {
            tokens: self.tokens,
        })
    }
}
