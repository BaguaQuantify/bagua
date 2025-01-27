use bagua_types::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum EventType {
    OnInit(OnInitEvent),
    OnStart(OnStartEvent),
    OnStop(OnStopEvent),

    OnDaily(OnDailyEvent),
    OnHourly(OnHourlyEvent),
    OnMinutely(OnMinutelyEvent),
    OnSecond(OnSecondEvent),
    OnTick(OnTickEvent),

    OnMarkPrice(OnMarkPriceEvent),
    OnIndexPrice(OnIndexPriceEvent),
    OnLastPrice(OnLastPriceEvent),
    OnFundingRate(OnFundingRateEvent),
    OnOrderbook(OnOrderbookEvent),
    OnCandle(OnCandleEvent),

    OnLeverage(OnOnLeverageEvent),
    OnOrder(OnOrderEvent),
    OnPosition(OnPositionEvent),
}

#[derive(Debug, Clone)]
pub struct OnInitEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnStartEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnStopEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnDailyEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnHourlyEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnMinutelyEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnSecondEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnTickEvent {
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OnMarkPriceEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub price: Price,
}

#[derive(Debug, Clone)]
pub struct OnIndexPriceEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub price: Price,
}

#[derive(Debug, Clone)]
pub struct OnLastPriceEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub price: Price,
}

#[derive(Debug, Clone)]
pub struct OnFundingRateEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub price: Price,
}

#[derive(Debug, Clone)]
pub struct OnOrderbookEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub orderbook: Orderbook,
}

#[derive(Debug, Clone)]
pub struct OnCandleEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub candle: Candle,
}

#[derive(Debug, Clone)]
pub struct OnOnLeverageEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub leverage: i32,
}

#[derive(Debug, Clone)]
pub struct OnOrderEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub order: Order,
}

#[derive(Debug, Clone)]
pub struct OnPositionEvent {
    pub product: ProductType,
    pub code: String,
    pub event_time: DateTime<Utc>,
    pub position: Position,
}
