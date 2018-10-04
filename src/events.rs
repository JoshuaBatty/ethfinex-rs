use book::{FundingCurrency as BookFundingCurrency, RawBook, TradingPair as BookTradingPair};
use candles::Candle;
use ticker::*;
use trades::{FundingCurrency as TradesFundingCurrency, TradingPair as TradesTradingPair};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum NotificationEvent {
    Info(InfoMessage),
    TradingSubscribed(TradingSubscriptionMessage),
    FundingSubscribed(FundingSubscriptionMessage),
    CandlesSubscribed(CandlesSubscriptionMessage),
    RawBookSubscribed(RawBookSubscriptionMessage),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DataEvent {
    TickerTradingEvent(u32, TradingPair),
    TickerFundingEvent(u32, FundingCurrency),
    TradesTradingSnapshotEvent(u32, Vec<TradesTradingPair>),
    TradesTradingUpdateEvent(u32, String, TradesTradingPair),
    TradesFundingSnapshotEvent(u32, Vec<TradesFundingCurrency>),
    TradesFundingUpdateEvent(u32, String, TradesFundingCurrency),
    BookTradingSnapshotEvent(u32, Vec<BookTradingPair>),
    BookTradingUpdateEvent(u32, BookTradingPair),
    BookFundingSnapshotEvent(u32, Vec<BookFundingCurrency>),
    BookFundingUpdateEvent(u32, BookFundingCurrency),
    RawBookUpdateEvent(u32, RawBook),
    RawBookSnapshotEvent(u32, Vec<RawBook>),
    CandlesSnapshotEvent(u32, Vec<Candle>),
    CandlesUpdateEvent(u32, Candle),
    HeartbeatEvent(u32, String),
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct InfoMessage {
    pub event: String,
    pub version: u16,
    pub server_id: String,
    pub platform: Platform,
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub status: u16,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct TradingSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub symbol: String,
    pub pair: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct FundingSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub symbol: String,
    pub currency: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct CandlesSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub key: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct RawBookSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub symbol: String,
    pub prec: String,
    pub freq: String,
    pub len: String,
    pub pair: String,
}
