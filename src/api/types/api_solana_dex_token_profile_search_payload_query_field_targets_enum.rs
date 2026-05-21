pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum {
    TokenAddress,
    MetadataName,
    MetadataSymbol,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TokenAddress => serializer.serialize_str("token_address"),
            Self::MetadataName => serializer.serialize_str("metadata.name"),
            Self::MetadataSymbol => serializer.serialize_str("metadata.symbol"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "token_address" => Ok(Self::TokenAddress),
            "metadata.name" => Ok(Self::MetadataName),
            "metadata.symbol" => Ok(Self::MetadataSymbol),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexTokenProfileSearchPayloadQueryFieldTargetsEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenAddress => write!(f, "token_address"),
            Self::MetadataName => write!(f, "metadata.name"),
            Self::MetadataSymbol => write!(f, "metadata.symbol"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
