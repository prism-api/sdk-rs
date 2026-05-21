pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexTokenProfileTimeWindowEnum {
    Window7D,
    Window1D,
    Window6H,
    Window1H,
    Window5M,
    Window1M,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexTokenProfileTimeWindowEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Window7D => serializer.serialize_str("7d"),
            Self::Window1D => serializer.serialize_str("1d"),
            Self::Window6H => serializer.serialize_str("6h"),
            Self::Window1H => serializer.serialize_str("1h"),
            Self::Window5M => serializer.serialize_str("5m"),
            Self::Window1M => serializer.serialize_str("1m"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexTokenProfileTimeWindowEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "7d" => Ok(Self::Window7D),
            "1d" => Ok(Self::Window1D),
            "6h" => Ok(Self::Window6H),
            "1h" => Ok(Self::Window1H),
            "5m" => Ok(Self::Window5M),
            "1m" => Ok(Self::Window1M),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexTokenProfileTimeWindowEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Window7D => write!(f, "7d"),
            Self::Window1D => write!(f, "1d"),
            Self::Window6H => write!(f, "6h"),
            Self::Window1H => write!(f, "1h"),
            Self::Window5M => write!(f, "5m"),
            Self::Window1M => write!(f, "1m"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
