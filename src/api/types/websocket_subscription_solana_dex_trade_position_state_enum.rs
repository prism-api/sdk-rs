pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexTradePositionStateEnum2 {
    Open,
    Close,
    Trade,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexTradePositionStateEnum2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Close => serializer.serialize_str("close"),
            Self::Trade => serializer.serialize_str("trade"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexTradePositionStateEnum2 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "close" => Ok(Self::Close),
            "trade" => Ok(Self::Trade),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexTradePositionStateEnum2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Close => write!(f, "close"),
            Self::Trade => write!(f, "trade"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
