pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UnsubscribePayloadParams {
        UnsubscribeSolanaDexPricesParams(UnsubscribeSolanaDexPricesParams),

        UnsubscribeSolanaDexSwapsParams(UnsubscribeSolanaDexSwapsParams),

        UnsubscribeSolanaDexTradesParams(UnsubscribeSolanaDexTradesParams),

        UnsubscribeSolanaDexWalletProfilesParams(UnsubscribeSolanaDexWalletProfilesParams),

        UnsubscribeSolanaDexTokenProfilesParams(UnsubscribeSolanaDexTokenProfilesParams),

        UnsubscribeSolanaDexPositionProfilesParams(UnsubscribeSolanaDexPositionProfilesParams),
}

impl UnsubscribePayloadParams {
    pub fn is_unsubscribe_solana_dex_prices_params(&self) -> bool {
        matches!(self, Self::UnsubscribeSolanaDexPricesParams(_))
    }

    pub fn is_unsubscribe_solana_dex_swaps_params(&self) -> bool {
        matches!(self, Self::UnsubscribeSolanaDexSwapsParams(_))
    }

    pub fn is_unsubscribe_solana_dex_trades_params(&self) -> bool {
        matches!(self, Self::UnsubscribeSolanaDexTradesParams(_))
    }

    pub fn is_unsubscribe_solana_dex_wallet_profiles_params(&self) -> bool {
        matches!(self, Self::UnsubscribeSolanaDexWalletProfilesParams(_))
    }

    pub fn is_unsubscribe_solana_dex_token_profiles_params(&self) -> bool {
        matches!(self, Self::UnsubscribeSolanaDexTokenProfilesParams(_))
    }

    pub fn is_unsubscribe_solana_dex_position_profiles_params(&self) -> bool {
        matches!(self, Self::UnsubscribeSolanaDexPositionProfilesParams(_))
    }


    pub fn as_unsubscribe_solana_dex_prices_params(&self) -> Option<&UnsubscribeSolanaDexPricesParams> {
        match self {
                    Self::UnsubscribeSolanaDexPricesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unsubscribe_solana_dex_prices_params(self) -> Option<UnsubscribeSolanaDexPricesParams> {
        match self {
                    Self::UnsubscribeSolanaDexPricesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_unsubscribe_solana_dex_swaps_params(&self) -> Option<&UnsubscribeSolanaDexSwapsParams> {
        match self {
                    Self::UnsubscribeSolanaDexSwapsParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unsubscribe_solana_dex_swaps_params(self) -> Option<UnsubscribeSolanaDexSwapsParams> {
        match self {
                    Self::UnsubscribeSolanaDexSwapsParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_unsubscribe_solana_dex_trades_params(&self) -> Option<&UnsubscribeSolanaDexTradesParams> {
        match self {
                    Self::UnsubscribeSolanaDexTradesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unsubscribe_solana_dex_trades_params(self) -> Option<UnsubscribeSolanaDexTradesParams> {
        match self {
                    Self::UnsubscribeSolanaDexTradesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_unsubscribe_solana_dex_wallet_profiles_params(&self) -> Option<&UnsubscribeSolanaDexWalletProfilesParams> {
        match self {
                    Self::UnsubscribeSolanaDexWalletProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unsubscribe_solana_dex_wallet_profiles_params(self) -> Option<UnsubscribeSolanaDexWalletProfilesParams> {
        match self {
                    Self::UnsubscribeSolanaDexWalletProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_unsubscribe_solana_dex_token_profiles_params(&self) -> Option<&UnsubscribeSolanaDexTokenProfilesParams> {
        match self {
                    Self::UnsubscribeSolanaDexTokenProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unsubscribe_solana_dex_token_profiles_params(self) -> Option<UnsubscribeSolanaDexTokenProfilesParams> {
        match self {
                    Self::UnsubscribeSolanaDexTokenProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_unsubscribe_solana_dex_position_profiles_params(&self) -> Option<&UnsubscribeSolanaDexPositionProfilesParams> {
        match self {
                    Self::UnsubscribeSolanaDexPositionProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_unsubscribe_solana_dex_position_profiles_params(self) -> Option<UnsubscribeSolanaDexPositionProfilesParams> {
        match self {
                    Self::UnsubscribeSolanaDexPositionProfilesParams(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UnsubscribePayloadParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsubscribeSolanaDexPricesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UnsubscribeSolanaDexSwapsParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UnsubscribeSolanaDexTradesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UnsubscribeSolanaDexWalletProfilesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UnsubscribeSolanaDexTokenProfilesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UnsubscribeSolanaDexPositionProfilesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
