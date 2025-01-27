use crate::event::ExchangeEvent;
use anyhow::Result;
use async_trait::async_trait;
use bagua_types::prelude::*;
use chrono::{DateTime, Utc};
use tokio::sync::mpsc::UnboundedReceiver;

#[async_trait]
pub trait Exchange: Clone + Send + Sync {
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

    async fn subscribe(&self) -> Result<UnboundedReceiver<ExchangeEvent>>;
}
