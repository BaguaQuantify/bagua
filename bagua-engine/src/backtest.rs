use crate::traits::Engine;
use anyhow::Result;
use async_trait::async_trait;
use bagua_types::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Backtest {
    pub curr_time: DateTime<Utc>,
}

#[async_trait]
impl Engine for Backtest {
    fn now(&self) -> DateTime<Utc> {
        self.curr_time
    }

    fn now_ms(&self) -> i64 {
        self.curr_time.timestamp_millis()
    }

    async fn get_candles(
        &self,
        product: ProductType,
        code: String,
        interval: CandleInterval,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<i64>,
    ) -> Result<Vec<Candle>> {
        todo!()
    }

    async fn get_positions(&self) -> Result<Vec<Position>> {
        todo!()
    }

    async fn get_leverage(&self, product: ProductType, code: String) -> Result<i32> {
        todo!()
    }

    async fn get_order(
        &self,
        product: ProductType,
        code: String,
        id: String,
    ) -> Result<Option<Order>> {
        todo!()
    }

    async fn get_open_orders(&self, product: ProductType, code: String) -> Result<Vec<Order>> {
        todo!()
    }

    async fn place_order(&self, order: Order) -> Result<()> {
        todo!()
    }

    async fn cancel_order(&self, product: ProductType, code: String, id: String) -> Result<()> {
        todo!()
    }

    async fn set_leverage(&self, product: ProductType, code: String, leverage: i32) -> Result<()> {
        todo!()
    }
}
