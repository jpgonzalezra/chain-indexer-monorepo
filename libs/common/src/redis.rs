use std::env;

use redis::Client;

pub const REDIS_HOST: &str = "REDIS_HOST";
pub const REDIS_PORT: &str = "REDIS_PORT";
pub const REDIS_PASSWORD: &str = "REDIS_PASSWORD";
pub const REDIS_DB: &str = "REDIS_DB";

pub fn redis_client_factory() -> Client {
    dotenv::dotenv().ok();
    let redis_password = env::var(REDIS_PASSWORD).unwrap_or_else(|_| "xxx".to_string());
    let redis_host = env::var(REDIS_HOST).unwrap_or_else(|_| "127.0.0.1".to_string());
    let redis_port = env::var(REDIS_PORT).unwrap_or_else(|_| "6379".to_string());
    let redis_db = env::var(REDIS_DB).unwrap_or_else(|_| "1".to_string());

    let redis_url = format!(
        "redis://:{}@{}:{}/{}",
        redis_password, redis_host, redis_port, redis_db
    );
    redis::Client::open(redis_url).expect("Error trying to connect to Redis")
}
