use std::env;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::{Client, RedisError};

pub const REDIS_HOST: &str = "REDIS_HOST";
pub const REDIS_PORT: &str = "REDIS_PORT";
pub const REDIS_PASSWORD: &str = "REDIS_PASSWORD";
pub const REDIS_DB: &str = "REDIS_DB";

pub async fn redis_pool_factory() -> Result<Pool<RedisConnectionManager>, RedisError> {
    let redis_url = build_redis_url()?;
    let manager = RedisConnectionManager::new(redis_url.as_str())?;
    Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .build(manager)
        .await
        .map_err(|e| e.into())
}

pub fn redis_client_factory() -> Result<Client, RedisError> {
    let redis_url = build_redis_url()?;
    redis::Client::open(redis_url)
}

fn build_redis_url() -> Result<String, RedisError> {
    dotenv::dotenv().ok();
    let redis_password = env::var(REDIS_PASSWORD).unwrap_or_default();
    let redis_host = env::var(REDIS_HOST).unwrap_or_else(|_| "127.0.0.1".to_string());
    let redis_port = env::var(REDIS_PORT).unwrap_or_else(|_| "6379".to_string());
    let redis_db = env::var(REDIS_DB).unwrap_or_else(|_| "0".to_string());

    Ok(format!(
        "redis://:{}@{}:{}/{}",
        redis_password, redis_host, redis_port, redis_db
    ))
}
