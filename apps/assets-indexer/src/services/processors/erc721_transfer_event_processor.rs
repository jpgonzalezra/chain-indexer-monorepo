use async_trait::async_trait;
use ethers::abi::{ethabi, ParamType};
use ethers::types::H256;

use super::event_processor::{EventProcessor, EventProcessorRequest, ProcessorError};
use super::event_processor_utils::decode_address;
use crate::services::repositories::erc721_repository::Erc721TransferTrait;
use crate::services::{
    processors::event_processor::ProcessResult,
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
        if event.topic0.to_lowercase() != Self::TRANSFER_TOPIC
            || event.topic1.is_none()
            || event.topic2.is_none()
            || event.topic3.is_none()
        {
            return Ok(ProcessResult::NotApplicable);
        }

        let from = decode_address(event.topic1.clone()).await?;
        let to = decode_address(event.topic2.clone()).await?;

        let id_bytes = array_bytes::hex_n_into::<String, H256, 32>(
            event.topic3.clone().ok_or_else(|| {
                ProcessorError::ValidationError("Missing 'id' topic".to_string())
            })?,
        )
        .map_err(|_| {
            ProcessorError::ParseError("Failed to parse 'id'".to_string(), "".to_string())
        })?;

        let id_tokens = ethabi::decode(&[ParamType::Uint(256)], id_bytes.as_bytes())
            .map_err(|e| {
                ProcessorError::DecodeError(
                    "Failed to decode 'id'".to_string(),
                    e.to_string(),
                )
            })?;

        let id = id_tokens.first().ok_or_else(|| {
            ProcessorError::ValidationError("Missing 'id' in tokens".to_string())
        })?;

        let id_uint = id.clone().into_uint().ok_or_else(|| {
            ProcessorError::ParseError(
                "Failed to convert token id into string".to_string(),
                "".to_string(),
            )
        })?;

        let contract_id = self
            .contract_repository
            .get_or_create_contract(&event.address, event.chain_id as i32)
            .await
            .map_err(|e| ProcessorError::DatabaseError(e.to_string()))?;

        let transfer_data = Erc721TransferData {
            contract_id,
            block_number: event.block_number as i32,
            chain_id: event.chain_id as i32,
            tx_hash: event.tx_hash.clone(),
            tx_index: event.tx_index,
            from,
            to,
            token_id: id_uint.to_string(),
        };

        self.erc721_repository
            .insert_transfer(transfer_data)
            .await
            .map_err(|e| ProcessorError::DatabaseError(e.to_string()))?;

        Ok(ProcessResult::Stored)
    }
}
