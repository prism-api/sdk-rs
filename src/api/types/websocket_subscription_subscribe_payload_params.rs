pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SubscribePayloadParams {
        SubscribeSolanaDexPricesParams(SubscribeSolanaDexPricesParams),

        SubscribeSolanaDexSwapsParams(SubscribeSolanaDexSwapsParams),

        SubscribeSolanaDexTradesParams(SubscribeSolanaDexTradesParams),

        SubscribeSolanaDexWalletProfilesParams(SubscribeSolanaDexWalletProfilesParams),

        SubscribeSolanaDexTokenProfilesParams(SubscribeSolanaDexTokenProfilesParams),

        SubscribeSolanaDexPositionProfilesParams(SubscribeSolanaDexPositionProfilesParams),
}

impl SubscribePayloadParams {
    pub fn is_subscribe_solana_dex_prices_params(&self) -> bool {
        matches!(self, Self::SubscribeSolanaDexPricesParams(_))
    }

    pub fn is_subscribe_solana_dex_swaps_params(&self) -> bool {
        matches!(self, Self::SubscribeSolanaDexSwapsParams(_))
    }

    pub fn is_subscribe_solana_dex_trades_params(&self) -> bool {
        matches!(self, Self::SubscribeSolanaDexTradesParams(_))
    }

    pub fn is_subscribe_solana_dex_wallet_profiles_params(&self) -> bool {
        matches!(self, Self::SubscribeSolanaDexWalletProfilesParams(_))
    }

    pub fn is_subscribe_solana_dex_token_profiles_params(&self) -> bool {
        matches!(self, Self::SubscribeSolanaDexTokenProfilesParams(_))
    }

    pub fn is_subscribe_solana_dex_position_profiles_params(&self) -> bool {
        matches!(self, Self::SubscribeSolanaDexPositionProfilesParams(_))
    }


    pub fn as_subscribe_solana_dex_prices_params(&self) -> Option<&SubscribeSolanaDexPricesParams> {
        match self {
                    Self::SubscribeSolanaDexPricesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subscribe_solana_dex_prices_params(self) -> Option<SubscribeSolanaDexPricesParams> {
        match self {
                    Self::SubscribeSolanaDexPricesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subscribe_solana_dex_swaps_params(&self) -> Option<&SubscribeSolanaDexSwapsParams> {
        match self {
                    Self::SubscribeSolanaDexSwapsParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subscribe_solana_dex_swaps_params(self) -> Option<SubscribeSolanaDexSwapsParams> {
        match self {
                    Self::SubscribeSolanaDexSwapsParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subscribe_solana_dex_trades_params(&self) -> Option<&SubscribeSolanaDexTradesParams> {
        match self {
                    Self::SubscribeSolanaDexTradesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subscribe_solana_dex_trades_params(self) -> Option<SubscribeSolanaDexTradesParams> {
        match self {
                    Self::SubscribeSolanaDexTradesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subscribe_solana_dex_wallet_profiles_params(&self) -> Option<&SubscribeSolanaDexWalletProfilesParams> {
        match self {
                    Self::SubscribeSolanaDexWalletProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subscribe_solana_dex_wallet_profiles_params(self) -> Option<SubscribeSolanaDexWalletProfilesParams> {
        match self {
                    Self::SubscribeSolanaDexWalletProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subscribe_solana_dex_token_profiles_params(&self) -> Option<&SubscribeSolanaDexTokenProfilesParams> {
        match self {
                    Self::SubscribeSolanaDexTokenProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subscribe_solana_dex_token_profiles_params(self) -> Option<SubscribeSolanaDexTokenProfilesParams> {
        match self {
                    Self::SubscribeSolanaDexTokenProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subscribe_solana_dex_position_profiles_params(&self) -> Option<&SubscribeSolanaDexPositionProfilesParams> {
        match self {
                    Self::SubscribeSolanaDexPositionProfilesParams(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subscribe_solana_dex_position_profiles_params(self) -> Option<SubscribeSolanaDexPositionProfilesParams> {
        match self {
                    Self::SubscribeSolanaDexPositionProfilesParams(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SubscribePayloadParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubscribeSolanaDexPricesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubscribeSolanaDexSwapsParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubscribeSolanaDexTradesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubscribeSolanaDexWalletProfilesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubscribeSolanaDexTokenProfilesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubscribeSolanaDexPositionProfilesParams(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
