pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SolanaDexTokenProfileMetrics2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub largest_trade_win: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub largest_trade_loss: Option<f64>,
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
    pub avg_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_buy_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_sell_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_buy_mcap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_sell_mcap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_holding_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_trade_pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_trade_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_wallet_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_wallet_pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_wallet_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_roi_distribution: Option<HashMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub makers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sellers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_position_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_position_count: Option<i64>,
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
    pub buy_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sell_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pnl: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume_weighted_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume_weighted_roi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wallet_hit_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub profit_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub win_loss_size_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pnl_volume_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_stability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_volatility: Option<f64>,
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
    pub pnl_volatility: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pnl_stability: Option<f64>,
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
}

impl SolanaDexTokenProfileMetrics2 {
    pub fn builder() -> SolanaDexTokenProfileMetrics2Builder {
        <SolanaDexTokenProfileMetrics2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SolanaDexTokenProfileMetrics2Builder {
    largest_trade_win: Option<f64>,
    largest_trade_loss: Option<f64>,
    avg_buy_size: Option<f64>,
    avg_sell_size: Option<f64>,
    avg_price: Option<f64>,
    avg_buy_price: Option<f64>,
    avg_sell_price: Option<f64>,
    avg_buy_mcap: Option<f64>,
    avg_sell_mcap: Option<f64>,
    avg_holding_duration: Option<i64>,
    avg_trade_pnl: Option<f64>,
    avg_trade_roi: Option<f64>,
    avg_wallet_winrate: Option<f64>,
    avg_wallet_pnl: Option<f64>,
    avg_wallet_roi: Option<f64>,
    wallet_roi_distribution: Option<HashMap<String, i64>>,
    makers_count: Option<i64>,
    buyers_count: Option<i64>,
    sellers_count: Option<i64>,
    buy_count: Option<i64>,
    sell_count: Option<i64>,
    win_position_count: Option<i64>,
    loss_position_count: Option<i64>,
    win_pnl: Option<f64>,
    loss_pnl: Option<f64>,
    buy_volume: Option<f64>,
    sell_volume: Option<f64>,
    pnl: Option<f64>,
    trade_count: Option<i64>,
    volume: Option<f64>,
    position_count: Option<i64>,
    volume_weighted_winrate: Option<f64>,
    volume_weighted_roi: Option<f64>,
    wallet_hit_ratio: Option<f64>,
    profit_factor: Option<f64>,
    win_loss_size_ratio: Option<f64>,
    pnl_volume_ratio: Option<f64>,
    price_stability: Option<f64>,
    price_volatility: Option<f64>,
    roi_stability: Option<f64>,
    roi_volatility: Option<f64>,
    pnl_volatility: Option<f64>,
    pnl_stability: Option<f64>,
    sharpe_ratio: Option<f64>,
    sortino_ratio: Option<f64>,
    max_drawdown: Option<f64>,
}

impl SolanaDexTokenProfileMetrics2Builder {
    pub fn largest_trade_win(mut self, value: f64) -> Self {
        self.largest_trade_win = Some(value);
        self
    }

    pub fn largest_trade_loss(mut self, value: f64) -> Self {
        self.largest_trade_loss = Some(value);
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

    pub fn avg_price(mut self, value: f64) -> Self {
        self.avg_price = Some(value);
        self
    }

    pub fn avg_buy_price(mut self, value: f64) -> Self {
        self.avg_buy_price = Some(value);
        self
    }

    pub fn avg_sell_price(mut self, value: f64) -> Self {
        self.avg_sell_price = Some(value);
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

    pub fn avg_holding_duration(mut self, value: i64) -> Self {
        self.avg_holding_duration = Some(value);
        self
    }

    pub fn avg_trade_pnl(mut self, value: f64) -> Self {
        self.avg_trade_pnl = Some(value);
        self
    }

    pub fn avg_trade_roi(mut self, value: f64) -> Self {
        self.avg_trade_roi = Some(value);
        self
    }

    pub fn avg_wallet_winrate(mut self, value: f64) -> Self {
        self.avg_wallet_winrate = Some(value);
        self
    }

    pub fn avg_wallet_pnl(mut self, value: f64) -> Self {
        self.avg_wallet_pnl = Some(value);
        self
    }

    pub fn avg_wallet_roi(mut self, value: f64) -> Self {
        self.avg_wallet_roi = Some(value);
        self
    }

    pub fn wallet_roi_distribution(mut self, value: HashMap<String, i64>) -> Self {
        self.wallet_roi_distribution = Some(value);
        self
    }

    pub fn makers_count(mut self, value: i64) -> Self {
        self.makers_count = Some(value);
        self
    }

    pub fn buyers_count(mut self, value: i64) -> Self {
        self.buyers_count = Some(value);
        self
    }

    pub fn sellers_count(mut self, value: i64) -> Self {
        self.sellers_count = Some(value);
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

    pub fn win_position_count(mut self, value: i64) -> Self {
        self.win_position_count = Some(value);
        self
    }

    pub fn loss_position_count(mut self, value: i64) -> Self {
        self.loss_position_count = Some(value);
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

    pub fn buy_volume(mut self, value: f64) -> Self {
        self.buy_volume = Some(value);
        self
    }

    pub fn sell_volume(mut self, value: f64) -> Self {
        self.sell_volume = Some(value);
        self
    }

    pub fn pnl(mut self, value: f64) -> Self {
        self.pnl = Some(value);
        self
    }

    pub fn trade_count(mut self, value: i64) -> Self {
        self.trade_count = Some(value);
        self
    }

    pub fn volume(mut self, value: f64) -> Self {
        self.volume = Some(value);
        self
    }

    pub fn position_count(mut self, value: i64) -> Self {
        self.position_count = Some(value);
        self
    }

    pub fn volume_weighted_winrate(mut self, value: f64) -> Self {
        self.volume_weighted_winrate = Some(value);
        self
    }

    pub fn volume_weighted_roi(mut self, value: f64) -> Self {
        self.volume_weighted_roi = Some(value);
        self
    }

    pub fn wallet_hit_ratio(mut self, value: f64) -> Self {
        self.wallet_hit_ratio = Some(value);
        self
    }

    pub fn profit_factor(mut self, value: f64) -> Self {
        self.profit_factor = Some(value);
        self
    }

    pub fn win_loss_size_ratio(mut self, value: f64) -> Self {
        self.win_loss_size_ratio = Some(value);
        self
    }

    pub fn pnl_volume_ratio(mut self, value: f64) -> Self {
        self.pnl_volume_ratio = Some(value);
        self
    }

    pub fn price_stability(mut self, value: f64) -> Self {
        self.price_stability = Some(value);
        self
    }

    pub fn price_volatility(mut self, value: f64) -> Self {
        self.price_volatility = Some(value);
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

    pub fn pnl_volatility(mut self, value: f64) -> Self {
        self.pnl_volatility = Some(value);
        self
    }

    pub fn pnl_stability(mut self, value: f64) -> Self {
        self.pnl_stability = Some(value);
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

    /// Consumes the builder and constructs a [`SolanaDexTokenProfileMetrics2`].
    pub fn build(self) -> Result<SolanaDexTokenProfileMetrics2, BuildError> {
        Ok(SolanaDexTokenProfileMetrics2 {
            largest_trade_win: self.largest_trade_win,
            largest_trade_loss: self.largest_trade_loss,
            avg_buy_size: self.avg_buy_size,
            avg_sell_size: self.avg_sell_size,
            avg_price: self.avg_price,
            avg_buy_price: self.avg_buy_price,
            avg_sell_price: self.avg_sell_price,
            avg_buy_mcap: self.avg_buy_mcap,
            avg_sell_mcap: self.avg_sell_mcap,
            avg_holding_duration: self.avg_holding_duration,
            avg_trade_pnl: self.avg_trade_pnl,
            avg_trade_roi: self.avg_trade_roi,
            avg_wallet_winrate: self.avg_wallet_winrate,
            avg_wallet_pnl: self.avg_wallet_pnl,
            avg_wallet_roi: self.avg_wallet_roi,
            wallet_roi_distribution: self.wallet_roi_distribution,
            makers_count: self.makers_count,
            buyers_count: self.buyers_count,
            sellers_count: self.sellers_count,
            buy_count: self.buy_count,
            sell_count: self.sell_count,
            win_position_count: self.win_position_count,
            loss_position_count: self.loss_position_count,
            win_pnl: self.win_pnl,
            loss_pnl: self.loss_pnl,
            buy_volume: self.buy_volume,
            sell_volume: self.sell_volume,
            pnl: self.pnl,
            trade_count: self.trade_count,
            volume: self.volume,
            position_count: self.position_count,
            volume_weighted_winrate: self.volume_weighted_winrate,
            volume_weighted_roi: self.volume_weighted_roi,
            wallet_hit_ratio: self.wallet_hit_ratio,
            profit_factor: self.profit_factor,
            win_loss_size_ratio: self.win_loss_size_ratio,
            pnl_volume_ratio: self.pnl_volume_ratio,
            price_stability: self.price_stability,
            price_volatility: self.price_volatility,
            roi_stability: self.roi_stability,
            roi_volatility: self.roi_volatility,
            pnl_volatility: self.pnl_volatility,
            pnl_stability: self.pnl_stability,
            sharpe_ratio: self.sharpe_ratio,
            sortino_ratio: self.sortino_ratio,
            max_drawdown: self.max_drawdown,
        })
    }
}
