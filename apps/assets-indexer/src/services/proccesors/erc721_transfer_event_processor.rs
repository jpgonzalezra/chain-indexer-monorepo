use async_trait::async_trait;

use super::event_processor::{EventProcessor, EventProcessorRequest, ProcessorError};
use crate::services::repositories::erc721_repository::Erc721TransferTrait;
use crate::services::{
    proccesors::event_processor::ProcessResult,
    repositories::{
        contract_repository::{ContractRepository, ContractRepositoryTrait},
        erc721_repository::{Erc721Repository, Erc721TransferData},
    },
};

pub struct Erc721TransferProcessor {
    pub erc721_repository: Erc721Repository,
    pub contract_repository: ContractRepository,
}

impl Erc721TransferProcessor {
    const TRANSFER_TOPIC: &'static str =
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
}

#[async_trait]
impl EventProcessor for Erc721TransferProcessor {
    async fn store_if_apply(
        &self,
        event: &EventProcessorRequest,
    ) -> Result<ProcessResult, ProcessorError> {
        if (event.topic0.to_lowercase() != Self::TRANSFER_TOPIC)
            && (event.topic1.is_none()
                || event.topic2.is_none()
                || event.topic3.is_none())
        {
            return Ok(ProcessResult::NotApplicable);
        }

        let from = event.topic1.as_ref().ok_or_else(|| {
            ProcessorError::ValidationError("Missing 'from' address".to_string())
        })?;
        let to = event.topic2.as_ref().ok_or_else(|| {
            ProcessorError::ValidationError("Missing 'to' address".to_string())
        })?;

        let token_id: i32 = i32::from_str_radix(&event.topic3.as_ref().unwrap()[2..], 16)
            .map_err(|_| {
                ProcessorError::DecodeError(
                    "Failed to parse token_id".to_string(),
                    event.topic3.clone().unwrap_or_default(),
                )
            })?;

        let contract_id = self
            .contract_repository
            .get_or_create_contract(&event.address, event.chain_id as i32)
            .await
            .map_err(|e| ProcessorError::DatabaseError(e.to_string()))?;

        self.erc721_repository
            .insert_transfer(Erc721TransferData {
                contract_id,
                block_number: event.block_number as i32,
                from: from.clone(),
                to: to.clone(),
                token_id,
            })
            .await
            .map_err(|e| ProcessorError::DatabaseError(e.to_string()))?;

        Ok(ProcessResult::Stored)
    }
}
