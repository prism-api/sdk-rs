pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SubscriptionMessageData {
        SolanaDexPrice(SolanaDexPrice2),

        SolanaDexSwap(SolanaDexSwap2),

        SolanaDexTrade(SolanaDexTrade2),

        SolanaDexWalletProfile(SolanaDexWalletProfile2),

        SolanaDexTokenProfile(SolanaDexTokenProfile2),
}

impl SubscriptionMessageData {
    pub fn is_solana_dex_price(&self) -> bool {
        matches!(self, Self::SolanaDexPrice(_))
    }

    pub fn is_solana_dex_swap(&self) -> bool {
        matches!(self, Self::SolanaDexSwap(_))
    }

    pub fn is_solana_dex_trade(&self) -> bool {
        matches!(self, Self::SolanaDexTrade(_))
    }

    pub fn is_solana_dex_wallet_profile(&self) -> bool {
        matches!(self, Self::SolanaDexWalletProfile(_))
    }

    pub fn is_solana_dex_token_profile(&self) -> bool {
        matches!(self, Self::SolanaDexTokenProfile(_))
    }


    pub fn as_solana_dex_price(&self) -> Option<&SolanaDexPrice2> {
        match self {
                    Self::SolanaDexPrice(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_price(self) -> Option<SolanaDexPrice2> {
        match self {
                    Self::SolanaDexPrice(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_swap(&self) -> Option<&SolanaDexSwap2> {
        match self {
                    Self::SolanaDexSwap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_swap(self) -> Option<SolanaDexSwap2> {
        match self {
                    Self::SolanaDexSwap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_trade(&self) -> Option<&SolanaDexTrade2> {
        match self {
                    Self::SolanaDexTrade(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_trade(self) -> Option<SolanaDexTrade2> {
        match self {
                    Self::SolanaDexTrade(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_wallet_profile(&self) -> Option<&SolanaDexWalletProfile2> {
        match self {
                    Self::SolanaDexWalletProfile(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_wallet_profile(self) -> Option<SolanaDexWalletProfile2> {
        match self {
                    Self::SolanaDexWalletProfile(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_token_profile(&self) -> Option<&SolanaDexTokenProfile2> {
        match self {
                    Self::SolanaDexTokenProfile(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_token_profile(self) -> Option<SolanaDexTokenProfile2> {
        match self {
                    Self::SolanaDexTokenProfile(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SubscriptionMessageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SolanaDexPrice(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexSwap(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexTrade(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexWalletProfile(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexTokenProfile(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
