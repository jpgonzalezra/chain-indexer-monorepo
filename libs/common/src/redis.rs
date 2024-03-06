use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::{Client, RedisError};

pub async fn redis_pool_factory(
    host: String,
    port: u16,
    password: Option<String>,
    db: usize,
) -> Result<Pool<RedisConnectionManager>, RedisError> {
    let redis_url = build_redis_url(host, port, password, db)?;
    let manager = RedisConnectionManager::new(redis_url.as_str())?;
    Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .build(manager)
        .await
}

pub fn redis_client_factory(
    host: String,
    port: u16,
    password: Option<String>,
    db: usize,
) -> Result<Client, RedisError> {
    let redis_url = build_redis_url(host, port, password, db)?;
    redis::Client::open(redis_url)
}

fn build_redis_url(
    host: String,
    port: u16,
    password: Option<String>,
    db: usize,
) -> Result<String, RedisError> {
    let password_part = password.map_or_else(|| "".to_string(), |p| format!(":{}@", p));
    let url = format!("redis://{}{}:{}/{}", password_part, host, port, db);
    Ok(url)
}
