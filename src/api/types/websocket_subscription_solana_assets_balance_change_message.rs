pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaAssetsBalanceChangeMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_type: Option<SolanaAssetsBalanceChangeTypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pre_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub post_balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub block_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

impl SolanaAssetsBalanceChangeMessage {
    pub fn builder() -> SolanaAssetsBalanceChangeMessageBuilder {
        <SolanaAssetsBalanceChangeMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaAssetsBalanceChangeMessageBuilder {
    id: Option<i64>,
    balance_type: Option<SolanaAssetsBalanceChangeTypeEnum>,
    stack_height: Option<i64>,
    token_address: Option<String>,
    owner_address: Option<String>,
    pre_balance: Option<f64>,
    post_balance: Option<f64>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
    tx_hash: Option<String>,
}

impl SolanaAssetsBalanceChangeMessageBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn balance_type(mut self, value: SolanaAssetsBalanceChangeTypeEnum) -> Self {
        self.balance_type = Some(value);
        self
    }

    pub fn stack_height(mut self, value: i64) -> Self {
        self.stack_height = Some(value);
        self
    }

    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn owner_address(mut self, value: impl Into<String>) -> Self {
        self.owner_address = Some(value.into());
        self
    }

    pub fn pre_balance(mut self, value: f64) -> Self {
        self.pre_balance = Some(value);
        self
    }

    pub fn post_balance(mut self, value: f64) -> Self {
        self.post_balance = Some(value);
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

    /// Consumes the builder and constructs a [`SolanaAssetsBalanceChangeMessage`].
    pub fn build(self) -> Result<SolanaAssetsBalanceChangeMessage, BuildError> {
        Ok(SolanaAssetsBalanceChangeMessage {
            id: self.id,
            balance_type: self.balance_type,
            stack_height: self.stack_height,
            token_address: self.token_address,
            owner_address: self.owner_address,
            pre_balance: self.pre_balance,
            post_balance: self.post_balance,
            block_slot: self.block_slot,
            block_time: self.block_time,
            tx_hash: self.tx_hash,
        })
    }
}
