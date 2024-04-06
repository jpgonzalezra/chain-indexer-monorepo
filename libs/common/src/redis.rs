use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::{Client, RedisError};

pub async fn redis_pool_factory(
    redis_url: String,
) -> Result<Pool<RedisConnectionManager>, RedisError> {
    let manager = RedisConnectionManager::new(redis_url.as_str())?;
    Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .build(manager)
        .await
}

pub fn redis_client_factory(redis_url: String) -> Result<Client, RedisError> {
    redis::Client::open(redis_url)
}
