use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Row;

#[async_trait]
pub trait ContractRepositoryTrait: Clone + Send + Sync + 'static {
    async fn get_or_create_contract(
        &self,
        address: &str,
        chain_id: i32,
    ) -> Result<i32, sqlx::Error>;
}

#[derive(Clone)]
pub struct ContractRepository {
    pub database_pool: Arc<PgPool>,
}

impl ContractRepository {
    pub fn new(database_pool: Arc<PgPool>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl ContractRepositoryTrait for ContractRepository {
    async fn get_or_create_contract(
        &self,
        address: &str,
        chain_id: i32,
    ) -> Result<i32, sqlx::Error> {
        let contract_result = sqlx::query_as::<_, (i32,)>(
            r#"
            SELECT id FROM contract WHERE address = $1 AND chain_id = $2
        "#,
        )
        .bind(address)
        .bind(chain_id)
        .fetch_optional(&*self.database_pool)
        .await?;

        if let Some((contract_id,)) = contract_result {
            Ok(contract_id)
        } else {
            sqlx::query(
                r#"
                INSERT INTO contract (address, chain_id, enabled) VALUES ($1, $2, TRUE)
                RETURNING id
            "#,
            )
            .bind(address)
            .bind(chain_id)
            .fetch_one(&*self.database_pool)
            .await
            .map(|row: sqlx::postgres::PgRow| row.get(0))
        }
    }
}
