pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexTrade {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<SolanaDexProtocolField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_type: Option<SolanaDexSwapTypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SolanaDexTradeDirectionEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_state: Option<SolanaDexTradePositionStateEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub quote_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub native_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pre_token_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub post_token_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub quote_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_mcap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub block_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

impl SolanaDexTrade {
    pub fn builder() -> SolanaDexTradeBuilder {
        <SolanaDexTradeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTradeBuilder {
    id: Option<i64>,
    protocol: Option<SolanaDexProtocolField>,
    swap_type: Option<SolanaDexSwapTypeEnum>,
    wallet_address: Option<String>,
    token_address: Option<String>,
    quote_address: Option<String>,
    position_address: Option<String>,
    direction: Option<SolanaDexTradeDirectionEnum>,
    position_state: Option<SolanaDexTradePositionStateEnum>,
    token_amount: Option<f64>,
    quote_amount: Option<f64>,
    native_amount: Option<f64>,
    usd_amount: Option<f64>,
    pre_token_balance: Option<f64>,
    post_token_balance: Option<f64>,
    token_price: Option<f64>,
    quote_price: Option<f64>,
    token_mcap: Option<f64>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
    tx_hash: Option<String>,
}

impl SolanaDexTradeBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn protocol(mut self, value: SolanaDexProtocolField) -> Self {
        self.protocol = Some(value);
        self
    }

    pub fn swap_type(mut self, value: SolanaDexSwapTypeEnum) -> Self {
        self.swap_type = Some(value);
        self
    }

    pub fn wallet_address(mut self, value: impl Into<String>) -> Self {
        self.wallet_address = Some(value.into());
        self
    }

    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn quote_address(mut self, value: impl Into<String>) -> Self {
        self.quote_address = Some(value.into());
        self
    }

    pub fn position_address(mut self, value: impl Into<String>) -> Self {
        self.position_address = Some(value.into());
        self
    }

    pub fn direction(mut self, value: SolanaDexTradeDirectionEnum) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn position_state(mut self, value: SolanaDexTradePositionStateEnum) -> Self {
        self.position_state = Some(value);
        self
    }

    pub fn token_amount(mut self, value: f64) -> Self {
        self.token_amount = Some(value);
        self
    }

    pub fn quote_amount(mut self, value: f64) -> Self {
        self.quote_amount = Some(value);
        self
    }

    pub fn native_amount(mut self, value: f64) -> Self {
        self.native_amount = Some(value);
        self
    }

    pub fn usd_amount(mut self, value: f64) -> Self {
        self.usd_amount = Some(value);
        self
    }

    pub fn pre_token_balance(mut self, value: f64) -> Self {
        self.pre_token_balance = Some(value);
        self
    }

    pub fn post_token_balance(mut self, value: f64) -> Self {
        self.post_token_balance = Some(value);
        self
    }

    pub fn token_price(mut self, value: f64) -> Self {
        self.token_price = Some(value);
        self
    }

    pub fn quote_price(mut self, value: f64) -> Self {
        self.quote_price = Some(value);
        self
    }

    pub fn token_mcap(mut self, value: f64) -> Self {
        self.token_mcap = Some(value);
        self
    }

    pub fn block_slot(mut self, value: i64) -> Self {
        self.block_slot = Some(value);
        self
    }

    pub fn block_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.block_time = Some(value);
        self
    }

    pub fn tx_hash(mut self, value: impl Into<String>) -> Self {
        self.tx_hash = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexTrade`].
    pub fn build(self) -> Result<SolanaDexTrade, BuildError> {
        Ok(SolanaDexTrade {
            id: self.id,
            protocol: self.protocol,
            swap_type: self.swap_type,
            wallet_address: self.wallet_address,
            token_address: self.token_address,
            quote_address: self.quote_address,
            position_address: self.position_address,
            direction: self.direction,
            position_state: self.position_state,
            token_amount: self.token_amount,
            quote_amount: self.quote_amount,
            native_amount: self.native_amount,
            usd_amount: self.usd_amount,
            pre_token_balance: self.pre_token_balance,
            post_token_balance: self.post_token_balance,
            token_price: self.token_price,
            quote_price: self.quote_price,
            token_mcap: self.token_mcap,
            block_slot: self.block_slot,
            block_time: self.block_time,
            tx_hash: self.tx_hash,
        })
    }
}
