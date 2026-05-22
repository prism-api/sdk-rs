pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexWalletProfileMetrics2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub risk_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub consistency_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub largest_win: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub largest_loss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_consecutive_wins: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_consecutive_losses: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_buy_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_sell_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_buy_mcap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_sell_mcap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_daily_pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_daily_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_daily_trade_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_daily_traded_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_daily_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_holding_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_trade_delta: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_token_pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_token_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_roi_distribution: Option<HashMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub win_pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub loss_pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub buy_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sell_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume_weighted_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume_weighted_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub time_weighted_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pnl_volume_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_hit_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub win_loss_size_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub profit_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub profit_expectancy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub recovery_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub winrate_stability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub winrate_volatility: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pnl_stability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pnl_volatility: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub roi_stability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub roi_volatility: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sharpe_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sortino_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_drawdown: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub risk_of_ruin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub turnover: Option<f64>,
}

impl SolanaDexWalletProfileMetrics2 {
    pub fn builder() -> SolanaDexWalletProfileMetrics2Builder {
        <SolanaDexWalletProfileMetrics2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexWalletProfileMetrics2Builder {
    risk_score: Option<f64>,
    consistency_score: Option<f64>,
    largest_win: Option<f64>,
    largest_loss: Option<f64>,
    max_consecutive_wins: Option<i64>,
    max_consecutive_losses: Option<i64>,
    avg_buy_size: Option<f64>,
    avg_sell_size: Option<f64>,
    avg_buy_mcap: Option<f64>,
    avg_sell_mcap: Option<f64>,
    avg_daily_pnl: Option<f64>,
    avg_daily_roi: Option<f64>,
    avg_daily_trade_count: Option<i64>,
    avg_daily_traded_tokens: Option<i64>,
    avg_daily_volume: Option<f64>,
    avg_holding_duration: Option<i64>,
    avg_trade_delta: Option<i64>,
    avg_pnl: Option<f64>,
    avg_roi: Option<f64>,
    avg_token_pnl: Option<f64>,
    avg_token_roi: Option<f64>,
    token_roi_distribution: Option<HashMap<String, i64>>,
    trade_count: Option<i64>,
    buy_count: Option<i64>,
    sell_count: Option<i64>,
    position_count: Option<i64>,
    win_count: Option<i64>,
    loss_count: Option<i64>,
    pnl: Option<f64>,
    win_pnl: Option<f64>,
    loss_pnl: Option<f64>,
    volume: Option<f64>,
    buy_volume: Option<f64>,
    sell_volume: Option<f64>,
    winrate: Option<f64>,
    volume_weighted_winrate: Option<f64>,
    roi: Option<f64>,
    volume_weighted_roi: Option<f64>,
    time_weighted_roi: Option<f64>,
    pnl_volume_ratio: Option<f64>,
    token_hit_ratio: Option<f64>,
    win_loss_size_ratio: Option<f64>,
    profit_factor: Option<f64>,
    profit_expectancy: Option<f64>,
    recovery_factor: Option<f64>,
    winrate_stability: Option<f64>,
    winrate_volatility: Option<f64>,
    pnl_stability: Option<f64>,
    pnl_volatility: Option<f64>,
    roi_stability: Option<f64>,
    roi_volatility: Option<f64>,
    sharpe_ratio: Option<f64>,
    sortino_ratio: Option<f64>,
    max_drawdown: Option<f64>,
    risk_of_ruin: Option<f64>,
    turnover: Option<f64>,
}

impl SolanaDexWalletProfileMetrics2Builder {
    pub fn risk_score(mut self, value: f64) -> Self {
        self.risk_score = Some(value);
        self
    }

    pub fn consistency_score(mut self, value: f64) -> Self {
        self.consistency_score = Some(value);
        self
    }

    pub fn largest_win(mut self, value: f64) -> Self {
        self.largest_win = Some(value);
        self
    }

    pub fn largest_loss(mut self, value: f64) -> Self {
        self.largest_loss = Some(value);
        self
    }

    pub fn max_consecutive_wins(mut self, value: i64) -> Self {
        self.max_consecutive_wins = Some(value);
        self
    }

    pub fn max_consecutive_losses(mut self, value: i64) -> Self {
        self.max_consecutive_losses = Some(value);
        self
    }

    pub fn avg_buy_size(mut self, value: f64) -> Self {
        self.avg_buy_size = Some(value);
        self
    }

    pub fn avg_sell_size(mut self, value: f64) -> Self {
        self.avg_sell_size = Some(value);
        self
    }

    pub fn avg_buy_mcap(mut self, value: f64) -> Self {
        self.avg_buy_mcap = Some(value);
        self
    }

    pub fn avg_sell_mcap(mut self, value: f64) -> Self {
        self.avg_sell_mcap = Some(value);
        self
    }

    pub fn avg_daily_pnl(mut self, value: f64) -> Self {
        self.avg_daily_pnl = Some(value);
        self
    }

    pub fn avg_daily_roi(mut self, value: f64) -> Self {
        self.avg_daily_roi = Some(value);
        self
    }

    pub fn avg_daily_trade_count(mut self, value: i64) -> Self {
        self.avg_daily_trade_count = Some(value);
        self
    }

    pub fn avg_daily_traded_tokens(mut self, value: i64) -> Self {
        self.avg_daily_traded_tokens = Some(value);
        self
    }

    pub fn avg_daily_volume(mut self, value: f64) -> Self {
        self.avg_daily_volume = Some(value);
        self
    }

    pub fn avg_holding_duration(mut self, value: i64) -> Self {
        self.avg_holding_duration = Some(value);
        self
    }

    pub fn avg_trade_delta(mut self, value: i64) -> Self {
        self.avg_trade_delta = Some(value);
        self
    }

    pub fn avg_pnl(mut self, value: f64) -> Self {
        self.avg_pnl = Some(value);
        self
    }

    pub fn avg_roi(mut self, value: f64) -> Self {
        self.avg_roi = Some(value);
        self
    }

    pub fn avg_token_pnl(mut self, value: f64) -> Self {
        self.avg_token_pnl = Some(value);
        self
    }

    pub fn avg_token_roi(mut self, value: f64) -> Self {
        self.avg_token_roi = Some(value);
        self
    }

    pub fn token_roi_distribution(mut self, value: HashMap<String, i64>) -> Self {
        self.token_roi_distribution = Some(value);
        self
    }

    pub fn trade_count(mut self, value: i64) -> Self {
        self.trade_count = Some(value);
        self
    }

    pub fn buy_count(mut self, value: i64) -> Self {
        self.buy_count = Some(value);
        self
    }

    pub fn sell_count(mut self, value: i64) -> Self {
        self.sell_count = Some(value);
        self
    }

    pub fn position_count(mut self, value: i64) -> Self {
        self.position_count = Some(value);
        self
    }

    pub fn win_count(mut self, value: i64) -> Self {
        self.win_count = Some(value);
        self
    }

    pub fn loss_count(mut self, value: i64) -> Self {
        self.loss_count = Some(value);
        self
    }

    pub fn pnl(mut self, value: f64) -> Self {
        self.pnl = Some(value);
        self
    }

    pub fn win_pnl(mut self, value: f64) -> Self {
        self.win_pnl = Some(value);
        self
    }

    pub fn loss_pnl(mut self, value: f64) -> Self {
        self.loss_pnl = Some(value);
        self
    }

    pub fn volume(mut self, value: f64) -> Self {
        self.volume = Some(value);
        self
    }

    pub fn buy_volume(mut self, value: f64) -> Self {
        self.buy_volume = Some(value);
        self
    }

    pub fn sell_volume(mut self, value: f64) -> Self {
        self.sell_volume = Some(value);
        self
    }

    pub fn winrate(mut self, value: f64) -> Self {
        self.winrate = Some(value);
        self
    }

    pub fn volume_weighted_winrate(mut self, value: f64) -> Self {
        self.volume_weighted_winrate = Some(value);
        self
    }

    pub fn roi(mut self, value: f64) -> Self {
        self.roi = Some(value);
        self
    }

    pub fn volume_weighted_roi(mut self, value: f64) -> Self {
        self.volume_weighted_roi = Some(value);
        self
    }

    pub fn time_weighted_roi(mut self, value: f64) -> Self {
        self.time_weighted_roi = Some(value);
        self
    }

    pub fn pnl_volume_ratio(mut self, value: f64) -> Self {
        self.pnl_volume_ratio = Some(value);
        self
    }

    pub fn token_hit_ratio(mut self, value: f64) -> Self {
        self.token_hit_ratio = Some(value);
        self
    }

    pub fn win_loss_size_ratio(mut self, value: f64) -> Self {
        self.win_loss_size_ratio = Some(value);
        self
    }

    pub fn profit_factor(mut self, value: f64) -> Self {
        self.profit_factor = Some(value);
        self
    }

    pub fn profit_expectancy(mut self, value: f64) -> Self {
        self.profit_expectancy = Some(value);
        self
    }

    pub fn recovery_factor(mut self, value: f64) -> Self {
        self.recovery_factor = Some(value);
        self
    }

    pub fn winrate_stability(mut self, value: f64) -> Self {
        self.winrate_stability = Some(value);
        self
    }

    pub fn winrate_volatility(mut self, value: f64) -> Self {
        self.winrate_volatility = Some(value);
        self
    }

    pub fn pnl_stability(mut self, value: f64) -> Self {
        self.pnl_stability = Some(value);
        self
    }

    pub fn pnl_volatility(mut self, value: f64) -> Self {
        self.pnl_volatility = Some(value);
        self
    }

    pub fn roi_stability(mut self, value: f64) -> Self {
        self.roi_stability = Some(value);
        self
    }

    pub fn roi_volatility(mut self, value: f64) -> Self {
        self.roi_volatility = Some(value);
        self
    }

    pub fn sharpe_ratio(mut self, value: f64) -> Self {
        self.sharpe_ratio = Some(value);
        self
    }

    pub fn sortino_ratio(mut self, value: f64) -> Self {
        self.sortino_ratio = Some(value);
        self
    }

    pub fn max_drawdown(mut self, value: f64) -> Self {
        self.max_drawdown = Some(value);
        self
    }

    pub fn risk_of_ruin(mut self, value: f64) -> Self {
        self.risk_of_ruin = Some(value);
        self
    }

    pub fn turnover(mut self, value: f64) -> Self {
        self.turnover = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SolanaDexWalletProfileMetrics2`].
    pub fn build(self) -> Result<SolanaDexWalletProfileMetrics2, BuildError> {
        Ok(SolanaDexWalletProfileMetrics2 {
            risk_score: self.risk_score,
            consistency_score: self.consistency_score,
            largest_win: self.largest_win,
            largest_loss: self.largest_loss,
            max_consecutive_wins: self.max_consecutive_wins,
            max_consecutive_losses: self.max_consecutive_losses,
            avg_buy_size: self.avg_buy_size,
            avg_sell_size: self.avg_sell_size,
            avg_buy_mcap: self.avg_buy_mcap,
            avg_sell_mcap: self.avg_sell_mcap,
            avg_daily_pnl: self.avg_daily_pnl,
            avg_daily_roi: self.avg_daily_roi,
            avg_daily_trade_count: self.avg_daily_trade_count,
            avg_daily_traded_tokens: self.avg_daily_traded_tokens,
            avg_daily_volume: self.avg_daily_volume,
            avg_holding_duration: self.avg_holding_duration,
            avg_trade_delta: self.avg_trade_delta,
            avg_pnl: self.avg_pnl,
            avg_roi: self.avg_roi,
            avg_token_pnl: self.avg_token_pnl,
            avg_token_roi: self.avg_token_roi,
            token_roi_distribution: self.token_roi_distribution,
            trade_count: self.trade_count,
            buy_count: self.buy_count,
            sell_count: self.sell_count,
            position_count: self.position_count,
            win_count: self.win_count,
            loss_count: self.loss_count,
            pnl: self.pnl,
            win_pnl: self.win_pnl,
            loss_pnl: self.loss_pnl,
            volume: self.volume,
            buy_volume: self.buy_volume,
            sell_volume: self.sell_volume,
            winrate: self.winrate,
            volume_weighted_winrate: self.volume_weighted_winrate,
            roi: self.roi,
            volume_weighted_roi: self.volume_weighted_roi,
            time_weighted_roi: self.time_weighted_roi,
            pnl_volume_ratio: self.pnl_volume_ratio,
            token_hit_ratio: self.token_hit_ratio,
            win_loss_size_ratio: self.win_loss_size_ratio,
            profit_factor: self.profit_factor,
            profit_expectancy: self.profit_expectancy,
            recovery_factor: self.recovery_factor,
            winrate_stability: self.winrate_stability,
            winrate_volatility: self.winrate_volatility,
            pnl_stability: self.pnl_stability,
            pnl_volatility: self.pnl_volatility,
            roi_stability: self.roi_stability,
            roi_volatility: self.roi_volatility,
            sharpe_ratio: self.sharpe_ratio,
            sortino_ratio: self.sortino_ratio,
            max_drawdown: self.max_drawdown,
            risk_of_ruin: self.risk_of_ruin,
            turnover: self.turnover,
        })
    }
}
