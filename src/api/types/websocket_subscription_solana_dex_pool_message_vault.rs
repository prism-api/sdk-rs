pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexPoolMessageVault {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
}

impl SolanaDexPoolMessageVault {
    pub fn builder() -> SolanaDexPoolMessageVaultBuilder {
        <SolanaDexPoolMessageVaultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexPoolMessageVaultBuilder {
    vault_address: Option<String>,
    token_address: Option<String>,
    amount: Option<f64>,
}

impl SolanaDexPoolMessageVaultBuilder {
    pub fn vault_address(mut self, value: impl Into<String>) -> Self {
        self.vault_address = Some(value.into());
        self
    }

    pub fn token_address(mut self, value: impl Into<String>) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexPoolMessageVault`].
    pub fn build(self) -> Result<SolanaDexPoolMessageVault, BuildError> {
        Ok(SolanaDexPoolMessageVault {
            vault_address: self.vault_address,
            token_address: self.token_address,
            amount: self.amount,
        })
    }
}
