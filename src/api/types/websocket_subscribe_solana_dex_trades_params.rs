pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubscribeSolanaDexTradesParams {
    pub tokens: Option<Vec<String>>,
}

impl SubscribeSolanaDexTradesParams {
    pub fn builder() -> SubscribeSolanaDexTradesParamsBuilder {
        <SubscribeSolanaDexTradesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeSolanaDexTradesParamsBuilder {
    tokens: Option<Vec<String>>,
}

impl SubscribeSolanaDexTradesParamsBuilder {
    pub fn tokens(mut self, value: Vec<String>) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeSolanaDexTradesParams`].
    pub fn build(self) -> Result<SubscribeSolanaDexTradesParams, BuildError> {
        Ok(SubscribeSolanaDexTradesParams {
            tokens: self.tokens,
        })
    }
}
