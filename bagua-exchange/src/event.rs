use bagua_types::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum ExchangeEvent {
    MarkPrice(PriceEvent),
    IndexPrice(PriceEvent),
    LastPrice(PriceEvent),
    FundingRate(PriceEvent),
    Orderbook(OrderbookEvent),
    Candle(CandleEvent),
    LeverageUpdate(LeverageUpdateEvent),
    OrderUpdate(OrderUpdateEvent),
    PositionUpdate(PositionUpdateEvent),
}

#[derive(Debug, Clone)]
pub struct PriceEvent {
    pub exchange: ExchangeType,
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub price: Price,
}

#[derive(Debug, Clone)]
pub struct OrderbookEvent {
    pub exchange: ExchangeType,
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub orderbook: Orderbook,
}

#[derive(Debug, Clone)]
pub struct CandleEvent {
    pub exchange: ExchangeType,
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub candle: Candle,
}

#[derive(Debug, Clone)]
pub struct LeverageUpdateEvent {
    pub exchange: ExchangeType,
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub leverage: i32,
}

#[derive(Debug, Clone)]
pub struct OrderUpdateEvent {
    pub exchange: ExchangeType,
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub order: Order,
}

#[derive(Debug, Clone)]
pub struct PositionUpdateEvent {
    pub exchange: ExchangeType,
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub position: Position,
}
