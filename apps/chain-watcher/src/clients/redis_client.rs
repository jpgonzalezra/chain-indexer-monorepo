use std::sync::Arc;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::RedisConnectionManager;
use common::types::SummaryLog;
use ethers::types::Log;
use redis::RedisError;

#[async_trait]
pub trait RedisClientTrait: Clone + Send + Sync + 'static {
    async fn send_logs(
        &self,
        key_stream: String,
        logs: Vec<Log>,
    ) -> Result<(), RedisError>;
}

#[derive(Clone)]
pub struct RedisClient {
    pub pool: Arc<Pool<RedisConnectionManager>>,
}

#[async_trait]
impl RedisClientTrait for RedisClient {
    async fn send_logs(
        &self,
        key_stream: String,
        logs: Vec<Log>,
    ) -> Result<(), RedisError> {
        if logs.is_empty() {
            return Ok(());
        }
        let pool_cloned = self.pool.clone();
        let mut conn = pool_cloned.get().await.expect("Pool connection Error");

        let summary_logs: Vec<SummaryLog> =
            logs.iter().map(|log| log.clone().into()).collect();
        let message: String = serde_json::to_string(&summary_logs).unwrap();

        conn.xadd(key_stream, "*", &[("message", &message)]).await
    }
}
