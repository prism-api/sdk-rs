pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexTokenProfileMarket {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub fdv: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub market_cap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub liquidity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holders: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_supply: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub circulating_supply: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub top_holdings: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dev_holdings: Option<f64>,
}

impl SolanaDexTokenProfileMarket {
    pub fn builder() -> SolanaDexTokenProfileMarketBuilder {
        <SolanaDexTokenProfileMarketBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTokenProfileMarketBuilder {
    fdv: Option<f64>,
    market_cap: Option<f64>,
    price: Option<f64>,
    liquidity: Option<f64>,
    holders: Option<i64>,
    total_supply: Option<f64>,
    circulating_supply: Option<f64>,
    top_holdings: Option<f64>,
    dev_holdings: Option<f64>,
}

impl SolanaDexTokenProfileMarketBuilder {
    pub fn fdv(mut self, value: f64) -> Self {
        self.fdv = Some(value);
        self
    }

    pub fn market_cap(mut self, value: f64) -> Self {
        self.market_cap = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn liquidity(mut self, value: f64) -> Self {
        self.liquidity = Some(value);
        self
    }

    pub fn holders(mut self, value: i64) -> Self {
        self.holders = Some(value);
        self
    }

    pub fn total_supply(mut self, value: f64) -> Self {
        self.total_supply = Some(value);
        self
    }

    pub fn circulating_supply(mut self, value: f64) -> Self {
        self.circulating_supply = Some(value);
        self
    }

    pub fn top_holdings(mut self, value: f64) -> Self {
        self.top_holdings = Some(value);
        self
    }

    pub fn dev_holdings(mut self, value: f64) -> Self {
        self.dev_holdings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexTokenProfileMarket`].
    pub fn build(self) -> Result<SolanaDexTokenProfileMarket, BuildError> {
        Ok(SolanaDexTokenProfileMarket {
            fdv: self.fdv,
            market_cap: self.market_cap,
            price: self.price,
            liquidity: self.liquidity,
            holders: self.holders,
            total_supply: self.total_supply,
            circulating_supply: self.circulating_supply,
            top_holdings: self.top_holdings,
            dev_holdings: self.dev_holdings,
        })
    }
}
