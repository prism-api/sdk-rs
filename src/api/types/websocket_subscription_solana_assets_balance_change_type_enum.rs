pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaAssetsBalanceChangeTypeEnum {
    SplTokenBalance,
    NativeBalance,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaAssetsBalanceChangeTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SplTokenBalance => serializer.serialize_str("spl_token_balance"),
            Self::NativeBalance => serializer.serialize_str("native_balance"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaAssetsBalanceChangeTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "spl_token_balance" => Ok(Self::SplTokenBalance),
            "native_balance" => Ok(Self::NativeBalance),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaAssetsBalanceChangeTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SplTokenBalance => write!(f, "spl_token_balance"),
            Self::NativeBalance => write!(f, "native_balance"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
