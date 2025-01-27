use crate::event::*;
use anyhow::Result;
use async_trait::async_trait;
use bagua_types::prelude::*;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait Strategy: Clone + Send + Sync {
    async fn on_init(&self, engine: impl Engine, event: OnInitEvent) -> Result<()>;
    async fn on_start(&self, engine: impl Engine, event: OnStartEvent) -> Result<()>;
    async fn on_stop(&self, engine: impl Engine, event: OnStopEvent) -> Result<()>;

    async fn on_daily(&self, engine: impl Engine, event: OnDailyEvent) -> Result<()>;
    async fn on_hourly(&self, engine: impl Engine, event: OnHourlyEvent) -> Result<()>;
    async fn on_minutely(&self, engine: impl Engine, event: OnMinutelyEvent) -> Result<()>;
    async fn on_second(&self, engine: impl Engine, event: OnSecondEvent) -> Result<()>;
    async fn on_tick(&self, engine: impl Engine, event: OnTickEvent) -> Result<()>;

    async fn on_mark_price(&self, engine: impl Engine, event: OnMarkPriceEvent) -> Result<()>;
    async fn on_index_price(&self, engine: impl Engine, event: OnIndexPriceEvent) -> Result<()>;
    async fn on_last_price(&self, engine: impl Engine, event: OnLastPriceEvent) -> Result<()>;
    async fn on_funding_rate(&self, engine: impl Engine, event: OnFundingRateEvent) -> Result<()>;
    async fn on_orderbook(&self, engine: impl Engine, event: OnOrderbookEvent) -> Result<()>;
    async fn on_candle(&self, engine: impl Engine, event: OnCandleEvent) -> Result<()>;

    async fn on_leverage(&self, engine: impl Engine, event: OnOnLeverageEvent) -> Result<()>;
    async fn on_order(&self, engine: impl Engine, event: OnOrderEvent) -> Result<()>;
    async fn on_position(&self, engine: impl Engine, event: OnPositionEvent) -> Result<()>;
}

#[async_trait]
pub trait Engine: Clone + Send + Sync {
    fn logo(&self) -> &str;
    fn now(&self) -> DateTime<Utc>;
    fn now_ms(&self) -> i64;
    fn ms_to_date(ms: i64) -> Result<DateTime<Utc>>;
    fn gen_id(&self) -> String;
    fn truncate_float(&self, val: f64, decimals: u32, round_up: bool) -> f64;
    fn is_zero(&self, val: f64) -> bool;
    async fn get_candles(
        &self,
        product: ProductType,
        code: String,
        interval: CandleInterval,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<i64>,
    ) -> Result<Vec<Candle>>;
    async fn get_positions(&self) -> Result<Vec<Position>>;
    async fn get_leverage(&self, product: ProductType, code: String) -> Result<i32>;
    async fn get_order(
        &self,
        product: ProductType,
        code: String,
        id: String,
    ) -> Result<Option<Order>>;
    async fn get_open_orders(&self, product: ProductType, code: String) -> Result<Vec<Order>>;
    async fn place_order(&self, order: Order) -> Result<()>;
    async fn cancel_order(&self, product: ProductType, code: String, id: String) -> Result<()>;
    async fn set_leverage(&self, product: ProductType, code: String, leverage: i32) -> Result<()>;
}
