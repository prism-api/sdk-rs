pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SubscriptionMessageData {
        SolanaDexPriceMessage(SolanaDexPriceMessage),

        SolanaDexSwapMessage(SolanaDexSwapMessage),

        SolanaDexTradeMessage(SolanaDexTradeMessage),

        SolanaDexPoolMessage(SolanaDexPoolMessage),

        SolanaDexWalletProfileMessage(SolanaDexWalletProfileMessage),

        SolanaDexTokenProfileMessage(SolanaDexTokenProfileMessage),

        SolanaAssetsTransferMessage(SolanaAssetsTransferMessage),

        SolanaAssetsBalanceChangeMessage(SolanaAssetsBalanceChangeMessage),
}

impl SubscriptionMessageData {
    pub fn is_solana_dex_price_message(&self) -> bool {
        matches!(self, Self::SolanaDexPriceMessage(_))
    }

    pub fn is_solana_dex_swap_message(&self) -> bool {
        matches!(self, Self::SolanaDexSwapMessage(_))
    }

    pub fn is_solana_dex_trade_message(&self) -> bool {
        matches!(self, Self::SolanaDexTradeMessage(_))
    }

    pub fn is_solana_dex_pool_message(&self) -> bool {
        matches!(self, Self::SolanaDexPoolMessage(_))
    }

    pub fn is_solana_dex_wallet_profile_message(&self) -> bool {
        matches!(self, Self::SolanaDexWalletProfileMessage(_))
    }

    pub fn is_solana_dex_token_profile_message(&self) -> bool {
        matches!(self, Self::SolanaDexTokenProfileMessage(_))
    }

    pub fn is_solana_assets_transfer_message(&self) -> bool {
        matches!(self, Self::SolanaAssetsTransferMessage(_))
    }

    pub fn is_solana_assets_balance_change_message(&self) -> bool {
        matches!(self, Self::SolanaAssetsBalanceChangeMessage(_))
    }


    pub fn as_solana_dex_price_message(&self) -> Option<&SolanaDexPriceMessage> {
        match self {
                    Self::SolanaDexPriceMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_price_message(self) -> Option<SolanaDexPriceMessage> {
        match self {
                    Self::SolanaDexPriceMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_swap_message(&self) -> Option<&SolanaDexSwapMessage> {
        match self {
                    Self::SolanaDexSwapMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_swap_message(self) -> Option<SolanaDexSwapMessage> {
        match self {
                    Self::SolanaDexSwapMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_trade_message(&self) -> Option<&SolanaDexTradeMessage> {
        match self {
                    Self::SolanaDexTradeMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_trade_message(self) -> Option<SolanaDexTradeMessage> {
        match self {
                    Self::SolanaDexTradeMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_pool_message(&self) -> Option<&SolanaDexPoolMessage> {
        match self {
                    Self::SolanaDexPoolMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_pool_message(self) -> Option<SolanaDexPoolMessage> {
        match self {
                    Self::SolanaDexPoolMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_wallet_profile_message(&self) -> Option<&SolanaDexWalletProfileMessage> {
        match self {
                    Self::SolanaDexWalletProfileMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_wallet_profile_message(self) -> Option<SolanaDexWalletProfileMessage> {
        match self {
                    Self::SolanaDexWalletProfileMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_dex_token_profile_message(&self) -> Option<&SolanaDexTokenProfileMessage> {
        match self {
                    Self::SolanaDexTokenProfileMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_dex_token_profile_message(self) -> Option<SolanaDexTokenProfileMessage> {
        match self {
                    Self::SolanaDexTokenProfileMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_assets_transfer_message(&self) -> Option<&SolanaAssetsTransferMessage> {
        match self {
                    Self::SolanaAssetsTransferMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_assets_transfer_message(self) -> Option<SolanaAssetsTransferMessage> {
        match self {
                    Self::SolanaAssetsTransferMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_solana_assets_balance_change_message(&self) -> Option<&SolanaAssetsBalanceChangeMessage> {
        match self {
                    Self::SolanaAssetsBalanceChangeMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_solana_assets_balance_change_message(self) -> Option<SolanaAssetsBalanceChangeMessage> {
        match self {
                    Self::SolanaAssetsBalanceChangeMessage(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SubscriptionMessageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SolanaDexPriceMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexSwapMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexTradeMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexPoolMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexWalletProfileMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaDexTokenProfileMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaAssetsTransferMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SolanaAssetsBalanceChangeMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
