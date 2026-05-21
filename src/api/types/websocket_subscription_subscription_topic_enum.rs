pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionTopicEnum {
    SolanaDexPrice,
    SolanaDexSwapWallet,
    SolanaDexSwapToken,
    SolanaDexTradeWallet,
    SolanaDexTradeToken,
    SolanaDexProfileWallet,
    SolanaDexProfileToken,
    SolanaDexProfilePosition,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionTopicEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SolanaDexPrice => serializer.serialize_str("solana.dex.price"),
            Self::SolanaDexSwapWallet => serializer.serialize_str("solana.dex.swap.wallet"),
            Self::SolanaDexSwapToken => serializer.serialize_str("solana.dex.swap.token"),
            Self::SolanaDexTradeWallet => serializer.serialize_str("solana.dex.trade.wallet"),
            Self::SolanaDexTradeToken => serializer.serialize_str("solana.dex.trade.token"),
            Self::SolanaDexProfileWallet => serializer.serialize_str("solana.dex.profile.wallet"),
            Self::SolanaDexProfileToken => serializer.serialize_str("solana.dex.profile.token"),
            Self::SolanaDexProfilePosition => serializer.serialize_str("solana.dex.profile.position"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionTopicEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "solana.dex.price" => Ok(Self::SolanaDexPrice),
            "solana.dex.swap.wallet" => Ok(Self::SolanaDexSwapWallet),
            "solana.dex.swap.token" => Ok(Self::SolanaDexSwapToken),
            "solana.dex.trade.wallet" => Ok(Self::SolanaDexTradeWallet),
            "solana.dex.trade.token" => Ok(Self::SolanaDexTradeToken),
            "solana.dex.profile.wallet" => Ok(Self::SolanaDexProfileWallet),
            "solana.dex.profile.token" => Ok(Self::SolanaDexProfileToken),
            "solana.dex.profile.position" => Ok(Self::SolanaDexProfilePosition),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionTopicEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SolanaDexPrice => write!(f, "solana.dex.price"),
            Self::SolanaDexSwapWallet => write!(f, "solana.dex.swap.wallet"),
            Self::SolanaDexSwapToken => write!(f, "solana.dex.swap.token"),
            Self::SolanaDexTradeWallet => write!(f, "solana.dex.trade.wallet"),
            Self::SolanaDexTradeToken => write!(f, "solana.dex.trade.token"),
            Self::SolanaDexProfileWallet => write!(f, "solana.dex.profile.wallet"),
            Self::SolanaDexProfileToken => write!(f, "solana.dex.profile.token"),
            Self::SolanaDexProfilePosition => write!(f, "solana.dex.profile.position"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
