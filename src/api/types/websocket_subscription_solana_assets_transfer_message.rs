pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaAssetsTransferMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<SolanaAssetsTransferTypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub block_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

impl SolanaAssetsTransferMessage {
    pub fn builder() -> SolanaAssetsTransferMessageBuilder {
        <SolanaAssetsTransferMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaAssetsTransferMessageBuilder {
    id: Option<i64>,
    transfer_type: Option<SolanaAssetsTransferTypeEnum>,
    stack_height: Option<i64>,
    token_address: Option<String>,
    from_address: Option<String>,
    to_address: Option<String>,
    amount: Option<f64>,
    usd_amount: Option<f64>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
    tx_hash: Option<String>,
}

impl SolanaAssetsTransferMessageBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn transfer_type(mut self, value: SolanaAssetsTransferTypeEnum) -> Self {
        self.transfer_type = Some(value);
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

    pub fn from_address(mut self, value: impl Into<String>) -> Self {
        self.from_address = Some(value.into());
        self
    }

    pub fn to_address(mut self, value: impl Into<String>) -> Self {
        self.to_address = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn usd_amount(mut self, value: f64) -> Self {
        self.usd_amount = Some(value);
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

    /// Consumes the builder and constructs a [`SolanaAssetsTransferMessage`].
    pub fn build(self) -> Result<SolanaAssetsTransferMessage, BuildError> {
        Ok(SolanaAssetsTransferMessage {
            id: self.id,
            transfer_type: self.transfer_type,
            stack_height: self.stack_height,
            token_address: self.token_address,
            from_address: self.from_address,
            to_address: self.to_address,
            amount: self.amount,
            usd_amount: self.usd_amount,
            block_slot: self.block_slot,
            block_time: self.block_time,
            tx_hash: self.tx_hash,
        })
    }
}
