pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexPoolMessageEventTypeEnum {
    Create,
    Swap,
    AddLiquidity,
    RemoveLiquidity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexPoolMessageEventTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Create => serializer.serialize_str("create"),
            Self::Swap => serializer.serialize_str("swap"),
            Self::AddLiquidity => serializer.serialize_str("add_liquidity"),
            Self::RemoveLiquidity => serializer.serialize_str("remove_liquidity"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexPoolMessageEventTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "create" => Ok(Self::Create),
            "swap" => Ok(Self::Swap),
            "add_liquidity" => Ok(Self::AddLiquidity),
            "remove_liquidity" => Ok(Self::RemoveLiquidity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexPoolMessageEventTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create => write!(f, "create"),
            Self::Swap => write!(f, "swap"),
            Self::AddLiquidity => write!(f, "add_liquidity"),
            Self::RemoveLiquidity => write!(f, "remove_liquidity"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
