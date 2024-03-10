use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::types::DbConfig;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(config: &DbConfig) -> Result<Self, sqlx::Error> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username,
            config.password.as_deref().unwrap_or(""),
            config.host,
            config.port,
            config.db_name
        );

        let pool = PgPoolOptions::new().connect(&database_url).await?;

        Ok(Database { pool })
    }
}
