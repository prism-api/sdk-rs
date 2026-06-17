pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionTopicEnum {
    SolanaDexPrices,
    SolanaDexSwaps,
    SolanaDexTrades,
    SolanaDexPools,
    SolanaDexProfilesWallet,
    SolanaDexProfilesToken,
    SolanaDexProfilesPosition,
    SolanaAssetsTransfers,
    SolanaAssetsBalanceChanges,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionTopicEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SolanaDexPrices => serializer.serialize_str("solana.dex.prices"),
            Self::SolanaDexSwaps => serializer.serialize_str("solana.dex.swaps"),
            Self::SolanaDexTrades => serializer.serialize_str("solana.dex.trades"),
            Self::SolanaDexPools => serializer.serialize_str("solana.dex.pools"),
            Self::SolanaDexProfilesWallet => serializer.serialize_str("solana.dex.profiles.wallet"),
            Self::SolanaDexProfilesToken => serializer.serialize_str("solana.dex.profiles.token"),
            Self::SolanaDexProfilesPosition => serializer.serialize_str("solana.dex.profiles.position"),
            Self::SolanaAssetsTransfers => serializer.serialize_str("solana.assets.transfers"),
            Self::SolanaAssetsBalanceChanges => serializer.serialize_str("solana.assets.balance-changes"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionTopicEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "solana.dex.prices" => Ok(Self::SolanaDexPrices),
            "solana.dex.swaps" => Ok(Self::SolanaDexSwaps),
            "solana.dex.trades" => Ok(Self::SolanaDexTrades),
            "solana.dex.pools" => Ok(Self::SolanaDexPools),
            "solana.dex.profiles.wallet" => Ok(Self::SolanaDexProfilesWallet),
            "solana.dex.profiles.token" => Ok(Self::SolanaDexProfilesToken),
            "solana.dex.profiles.position" => Ok(Self::SolanaDexProfilesPosition),
            "solana.assets.transfers" => Ok(Self::SolanaAssetsTransfers),
            "solana.assets.balance-changes" => Ok(Self::SolanaAssetsBalanceChanges),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionTopicEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SolanaDexPrices => write!(f, "solana.dex.prices"),
            Self::SolanaDexSwaps => write!(f, "solana.dex.swaps"),
            Self::SolanaDexTrades => write!(f, "solana.dex.trades"),
            Self::SolanaDexPools => write!(f, "solana.dex.pools"),
            Self::SolanaDexProfilesWallet => write!(f, "solana.dex.profiles.wallet"),
            Self::SolanaDexProfilesToken => write!(f, "solana.dex.profiles.token"),
            Self::SolanaDexProfilesPosition => write!(f, "solana.dex.profiles.position"),
            Self::SolanaAssetsTransfers => write!(f, "solana.assets.transfers"),
            Self::SolanaAssetsBalanceChanges => write!(f, "solana.assets.balance-changes"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
