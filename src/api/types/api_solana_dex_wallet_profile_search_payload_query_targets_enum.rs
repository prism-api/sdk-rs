pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexWalletProfileSearchPayloadQueryTargetsEnum {
    WalletAddress,
    IdentityName,
    IdentityTwitterUsername,
    IdentityTelegramUsername,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexWalletProfileSearchPayloadQueryTargetsEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WalletAddress => serializer.serialize_str("wallet_address"),
            Self::IdentityName => serializer.serialize_str("identity.name"),
            Self::IdentityTwitterUsername => serializer.serialize_str("identity.twitter_username"),
            Self::IdentityTelegramUsername => serializer.serialize_str("identity.telegram_username"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexWalletProfileSearchPayloadQueryTargetsEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wallet_address" => Ok(Self::WalletAddress),
            "identity.name" => Ok(Self::IdentityName),
            "identity.twitter_username" => Ok(Self::IdentityTwitterUsername),
            "identity.telegram_username" => Ok(Self::IdentityTelegramUsername),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexWalletProfileSearchPayloadQueryTargetsEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WalletAddress => write!(f, "wallet_address"),
            Self::IdentityName => write!(f, "identity.name"),
            Self::IdentityTwitterUsername => write!(f, "identity.twitter_username"),
            Self::IdentityTelegramUsername => write!(f, "identity.telegram_username"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
