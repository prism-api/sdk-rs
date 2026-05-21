pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexWalletProfileTimeWindowEnum {
    Window30D,
    Window14D,
    Window7D,
    Window3D,
    Window1D,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexWalletProfileTimeWindowEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Window30D => serializer.serialize_str("30d"),
            Self::Window14D => serializer.serialize_str("14d"),
            Self::Window7D => serializer.serialize_str("7d"),
            Self::Window3D => serializer.serialize_str("3d"),
            Self::Window1D => serializer.serialize_str("1d"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexWalletProfileTimeWindowEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "30d" => Ok(Self::Window30D),
            "14d" => Ok(Self::Window14D),
            "7d" => Ok(Self::Window7D),
            "3d" => Ok(Self::Window3D),
            "1d" => Ok(Self::Window1D),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexWalletProfileTimeWindowEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Window30D => write!(f, "30d"),
            Self::Window14D => write!(f, "14d"),
            Self::Window7D => write!(f, "7d"),
            Self::Window3D => write!(f, "3d"),
            Self::Window1D => write!(f, "1d"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
