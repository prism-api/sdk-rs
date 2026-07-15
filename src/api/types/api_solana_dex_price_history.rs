pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPriceHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<SolanaDexPriceSnapshot>>,
}

impl SolanaDexPriceHistory {
    pub fn builder() -> SolanaDexPriceHistoryBuilder {
        <SolanaDexPriceHistoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPriceHistoryBuilder {
    token_address: Option<String>,
    pool_address: Option<String>,
    prices: Option<Vec<SolanaDexPriceSnapshot>>,
}

impl SolanaDexPriceHistoryBuilder {
    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn pool_address(mut self, value: impl Into<String>) -> Self {
        self.pool_address = Some(value.into());
        self
    }

    pub fn prices(mut self, value: Vec<SolanaDexPriceSnapshot>) -> Self {
        self.prices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPriceHistory`].
    pub fn build(self) -> Result<SolanaDexPriceHistory, BuildError> {
        Ok(SolanaDexPriceHistory {
            token_address: self.token_address,
            pool_address: self.pool_address,
            prices: self.prices,
        })
    }
}
