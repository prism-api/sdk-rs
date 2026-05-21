pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolanaDexSwapTypeEnum2 {
    QuoteToken,
    TokenQuote,
    TokenToken,
    QuoteQuote,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SolanaDexSwapTypeEnum2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::QuoteToken => serializer.serialize_str("quote_token"),
            Self::TokenQuote => serializer.serialize_str("token_quote"),
            Self::TokenToken => serializer.serialize_str("token_token"),
            Self::QuoteQuote => serializer.serialize_str("quote_quote"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SolanaDexSwapTypeEnum2 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "quote_token" => Ok(Self::QuoteToken),
            "token_quote" => Ok(Self::TokenQuote),
            "token_token" => Ok(Self::TokenToken),
            "quote_quote" => Ok(Self::QuoteQuote),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SolanaDexSwapTypeEnum2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QuoteToken => write!(f, "quote_token"),
            Self::TokenQuote => write!(f, "token_quote"),
            Self::TokenToken => write!(f, "token_token"),
            Self::QuoteQuote => write!(f, "quote_quote"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
