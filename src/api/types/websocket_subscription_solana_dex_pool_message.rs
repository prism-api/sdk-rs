pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPoolMessage {
    /// Refer to [Data Sources](/documentation/solana/dex/overview#data-sources) for the list of supported protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<SolanaDexPoolMessageEventTypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vaults: Option<Vec<SolanaDexPoolMessageVault>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_slot: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub block_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

impl SolanaDexPoolMessage {
    pub fn builder() -> SolanaDexPoolMessageBuilder {
        <SolanaDexPoolMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPoolMessageBuilder {
    protocol: Option<String>,
    event_type: Option<SolanaDexPoolMessageEventTypeEnum>,
    program_address: Option<String>,
    pool_address: Option<String>,
    vaults: Option<Vec<SolanaDexPoolMessageVault>>,
    block_slot: Option<i64>,
    block_time: Option<DateTime<FixedOffset>>,
    tx_hash: Option<String>,
}

impl SolanaDexPoolMessageBuilder {
    pub fn protocol(mut self, value: impl Into<String>) -> Self {
        self.protocol = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: SolanaDexPoolMessageEventTypeEnum) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn program_address(mut self, value: impl Into<String>) -> Self {
        self.program_address = Some(value.into());
        self
    }

    pub fn pool_address(mut self, value: impl Into<String>) -> Self {
        self.pool_address = Some(value.into());
        self
    }

    pub fn vaults(mut self, value: Vec<SolanaDexPoolMessageVault>) -> Self {
        self.vaults = Some(value);
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

    /// Consumes the builder and constructs a [`SolanaDexPoolMessage`].
    pub fn build(self) -> Result<SolanaDexPoolMessage, BuildError> {
        Ok(SolanaDexPoolMessage {
            protocol: self.protocol,
            event_type: self.event_type,
            program_address: self.program_address,
            pool_address: self.pool_address,
            vaults: self.vaults,
            block_slot: self.block_slot,
            block_time: self.block_time,
            tx_hash: self.tx_hash,
        })
    }
}
