use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Pair = String;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Timeframe {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
}

impl Timeframe {
    pub fn as_str(&self) -> &'static str {
        match self {
            Timeframe::OneMinute => "1m",
            Timeframe::ThreeMinutes => "3m",
            Timeframe::FiveMinutes => "5m",
            Timeframe::FifteenMinutes => "15m",
            Timeframe::ThirtyMinutes => "30m",
            Timeframe::OneHour => "1h",
            Timeframe::TwoHours => "2h",
            Timeframe::FourHours => "4h",
            Timeframe::SixHours => "6h",
            Timeframe::EightHours => "8h",
            Timeframe::TwelveHours => "12h",
            Timeframe::OneDay => "1d",
            Timeframe::ThreeDays => "3d",
            Timeframe::OneWeek => "1w",
            Timeframe::OneMonth => "1M",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    StopLimit,
    StopMarket,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExitType {
    Signal,
    StopLoss,
    TakeProfit,
    StopLossOnExchange,
    ForceExit,
    EmergencyExit,
    Custom,
}

impl std::fmt::Display for ExitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExitType::Signal => write!(f, "signal"),
            ExitType::StopLoss => write!(f, "stop_loss"),
            ExitType::TakeProfit => write!(f, "take_profit"),
            ExitType::StopLossOnExchange => write!(f, "stop_loss_on_exchange"),
            ExitType::ForceExit => write!(f, "force_exit"),
            ExitType::EmergencyExit => write!(f, "emergency_exit"),
            ExitType::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCV {
    pub timestamp: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub price: Decimal,
    pub volume_24h: Decimal,
    pub change_24h: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub currency: String,
    pub total: Decimal,
    pub free: Decimal,
    pub used: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub side: TradeSide,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub price: Option<Decimal>,
    pub amount: Decimal,
    pub filled: Decimal,
    pub remaining: Decimal,
    pub fee: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub pair: Pair,
    pub is_open: bool,
    pub exchange: String,
    pub open_rate: Decimal,
    pub open_date: DateTime<Utc>,
    pub close_rate: Option<Decimal>,
    pub close_date: Option<DateTime<Utc>>,
    pub amount: Decimal,
    pub stake_amount: Decimal,
    pub strategy: String,
    pub timeframe: Timeframe,
    pub stop_loss: Option<Decimal>,
    pub take_profit: Option<Decimal>,
    pub exit_reason: Option<ExitType>,
    pub profit_abs: Option<Decimal>,
    pub profit_ratio: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub side: TradeSide,
    pub order_type: OrderType,
    pub amount: Decimal,
    pub price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub side: TradeSide,
    pub size: Decimal,
    pub entry_price: Decimal,
    pub mark_price: Decimal,
    pub unrealized_pnl: Decimal,
    pub percentage: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub index: usize,
    pub r#type: SignalType,
    pub strength: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SignalType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub strategy: String,
    pub pair: String,
    pub timeframe: Timeframe,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub total_profit: Decimal,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub profit_factor: f64,
    pub avg_profit: Decimal,
    pub avg_loss: Decimal,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BotStatus {
    Stopped,
    Running,
    Paused,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotState {
    pub status: BotStatus,
    pub open_trades: Vec<Trade>,
    pub closed_trades: Vec<Trade>,
    pub balance: Balance,
    pub last_update: DateTime<Utc>,
    pub current_pair: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_profit: f64,
    pub win_rate: f64,
    pub open_trades: usize,
    pub max_drawdown: f64,
    pub total_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    pub time: String,
    pub value: f64,
}
