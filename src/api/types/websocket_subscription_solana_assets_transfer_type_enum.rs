pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaAssetsTransferTypeEnum {
    SplTokenTransfer,
    NativeTransfer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaAssetsTransferTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SplTokenTransfer => serializer.serialize_str("spl_token_transfer"),
            Self::NativeTransfer => serializer.serialize_str("native_transfer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaAssetsTransferTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "spl_token_transfer" => Ok(Self::SplTokenTransfer),
            "native_transfer" => Ok(Self::NativeTransfer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaAssetsTransferTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SplTokenTransfer => write!(f, "spl_token_transfer"),
            Self::NativeTransfer => write!(f, "native_transfer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
