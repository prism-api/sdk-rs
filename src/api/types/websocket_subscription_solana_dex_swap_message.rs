pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexSwapMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_type: Option<SolanaDexSwapTypeEnum2>,
    /// Refer to [Data Sources](/documentation/solana/dex/overview#data-sources) for the list of supported protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_amount_in: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_amount_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_price_in: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_price_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pre_token_balance_in: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pre_token_balance_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub post_token_balance_in: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub post_token_balance_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_amount_in: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_amount_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub block_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

impl SolanaDexSwapMessage {
    pub fn builder() -> SolanaDexSwapMessageBuilder {
        <SolanaDexSwapMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexSwapMessageBuilder {
    id: Option<i64>,
    swap_type: Option<SolanaDexSwapTypeEnum2>,
    protocol: Option<String>,
    wallet_address: Option<String>,
    token_address_in: Option<String>,
    token_address_out: Option<String>,
    token_amount_in: Option<f64>,
    token_amount_out: Option<f64>,
    token_price_in: Option<f64>,
    token_price_out: Option<f64>,
    pre_token_balance_in: Option<f64>,
    pre_token_balance_out: Option<f64>,
    post_token_balance_in: Option<f64>,
    post_token_balance_out: Option<f64>,
    usd_amount_in: Option<f64>,
    usd_amount_out: Option<f64>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
    tx_hash: Option<String>,
}

impl SolanaDexSwapMessageBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn swap_type(mut self, value: SolanaDexSwapTypeEnum2) -> Self {
        self.swap_type = Some(value);
        self
    }

    pub fn protocol(mut self, value: impl Into<String>) -> Self {
        self.protocol = Some(value.into());
        self
    }

    pub fn wallet_address(mut self, value: impl Into<String>) -> Self {
        self.wallet_address = Some(value.into());
        self
    }

    pub fn token_address_in(mut self, value: impl Into<String>) -> Self {
        self.token_address_in = Some(value.into());
        self
    }

    pub fn token_address_out(mut self, value: impl Into<String>) -> Self {
        self.token_address_out = Some(value.into());
        self
    }

    pub fn token_amount_in(mut self, value: f64) -> Self {
        self.token_amount_in = Some(value);
        self
    }

    pub fn token_amount_out(mut self, value: f64) -> Self {
        self.token_amount_out = Some(value);
        self
    }

    pub fn token_price_in(mut self, value: f64) -> Self {
        self.token_price_in = Some(value);
        self
    }

    pub fn token_price_out(mut self, value: f64) -> Self {
        self.token_price_out = Some(value);
        self
    }

    pub fn pre_token_balance_in(mut self, value: f64) -> Self {
        self.pre_token_balance_in = Some(value);
        self
    }

    pub fn pre_token_balance_out(mut self, value: f64) -> Self {
        self.pre_token_balance_out = Some(value);
        self
    }

    pub fn post_token_balance_in(mut self, value: f64) -> Self {
        self.post_token_balance_in = Some(value);
        self
    }

    pub fn post_token_balance_out(mut self, value: f64) -> Self {
        self.post_token_balance_out = Some(value);
        self
    }

    pub fn usd_amount_in(mut self, value: f64) -> Self {
        self.usd_amount_in = Some(value);
        self
    }

    pub fn usd_amount_out(mut self, value: f64) -> Self {
        self.usd_amount_out = Some(value);
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

    /// Consumes the builder and constructs a [`SolanaDexSwapMessage`].
    pub fn build(self) -> Result<SolanaDexSwapMessage, BuildError> {
        Ok(SolanaDexSwapMessage {
            id: self.id,
            swap_type: self.swap_type,
            protocol: self.protocol,
            wallet_address: self.wallet_address,
            token_address_in: self.token_address_in,
            token_address_out: self.token_address_out,
            token_amount_in: self.token_amount_in,
            token_amount_out: self.token_amount_out,
            token_price_in: self.token_price_in,
            token_price_out: self.token_price_out,
            pre_token_balance_in: self.pre_token_balance_in,
            pre_token_balance_out: self.pre_token_balance_out,
            post_token_balance_in: self.post_token_balance_in,
            post_token_balance_out: self.post_token_balance_out,
            usd_amount_in: self.usd_amount_in,
            usd_amount_out: self.usd_amount_out,
            block_slot: self.block_slot,
            block_time: self.block_time,
            tx_hash: self.tx_hash,
        })
    }
}
