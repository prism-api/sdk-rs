pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexWalletProfileLabelEnum {
    Kol,
    Trencher,
    Conviction,
    Smart,
    Whale,
    Degen,
    Sniper,
    Swing,
    Holder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexWalletProfileLabelEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Kol => serializer.serialize_str("kol"),
            Self::Trencher => serializer.serialize_str("trencher"),
            Self::Conviction => serializer.serialize_str("conviction"),
            Self::Smart => serializer.serialize_str("smart"),
            Self::Whale => serializer.serialize_str("whale"),
            Self::Degen => serializer.serialize_str("degen"),
            Self::Sniper => serializer.serialize_str("sniper"),
            Self::Swing => serializer.serialize_str("swing"),
            Self::Holder => serializer.serialize_str("holder"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexWalletProfileLabelEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "kol" => Ok(Self::Kol),
            "trencher" => Ok(Self::Trencher),
            "conviction" => Ok(Self::Conviction),
            "smart" => Ok(Self::Smart),
            "whale" => Ok(Self::Whale),
            "degen" => Ok(Self::Degen),
            "sniper" => Ok(Self::Sniper),
            "swing" => Ok(Self::Swing),
            "holder" => Ok(Self::Holder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexWalletProfileLabelEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kol => write!(f, "kol"),
            Self::Trencher => write!(f, "trencher"),
            Self::Conviction => write!(f, "conviction"),
            Self::Smart => write!(f, "smart"),
            Self::Whale => write!(f, "whale"),
            Self::Degen => write!(f, "degen"),
            Self::Sniper => write!(f, "sniper"),
            Self::Swing => write!(f, "swing"),
            Self::Holder => write!(f, "holder"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
