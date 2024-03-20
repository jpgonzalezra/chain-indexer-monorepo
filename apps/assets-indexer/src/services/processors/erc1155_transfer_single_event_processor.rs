use async_trait::async_trait;
use ethers::types::H256;

use super::event_processor::{
    EventProcessor, EventProcessorRequest, ProcessResult, ProcessorError,
};
use crate::services::repositories::contract_repository::ContractRepositoryTrait;
use crate::services::repositories::erc1155_repository::Erc1155TransferTrait;
use crate::services::repositories::{
    contract_repository::ContractRepository,
    erc1155_repository::{Erc1155Repository, Erc1155TransferData},
};
use ethers::{
    abi::{ethabi, ParamType},
    utils::hex,
};
pub struct Erc1155TransferSingleProcessor {
    pub erc1155_repository: Erc1155Repository,
    pub contract_repository: ContractRepository,
}

impl Erc1155TransferSingleProcessor {
    const ERC1155_TRANSFER_SINGLE_EVENT_SIGNATURE: &'static str =
        "0xc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62";
}

#[async_trait]
impl EventProcessor for Erc1155TransferSingleProcessor {
    async fn store_if_apply(
        &self,
        event: &EventProcessorRequest,
    ) -> Result<ProcessResult, ProcessorError> {
        if event.topic0.to_lowercase() != Self::ERC1155_TRANSFER_SINGLE_EVENT_SIGNATURE
            || event.topic1.is_none()
            || event.topic2.is_none()
            || event.topic3.is_none()
        {
            return Ok(ProcessResult::NotApplicable);
        }

        let data = hex::decode(&event.data[2..]).map_err(|e| {
            ProcessorError::DecodeError(e.to_string(), event.data.clone())
        })?;

        let transfer_values =
            ethabi::decode(&[ParamType::Uint(256), ParamType::Uint(256)], &data[..])
                .map_err(|e| {
                    ProcessorError::DecodeError(e.to_string(), event.data.clone())
                })?;

        let contract_id = self
            .contract_repository
            .get_or_create_contract(&event.address, event.chain_id as i32)
            .await
            .map_err(|e| ProcessorError::DatabaseError(e.to_string()))?;

        let from_address_bytes = array_bytes::hex_n_into::<String, H256, 32>(
            event.topic2.clone().ok_or_else(|| {
                ProcessorError::ValidationError(
                    "Missing 'from' address topic".to_string(),
                )
            })?,
        )
        .map_err(|_| {
            ProcessorError::ParseError(
                "Failed to parse 'from' address".to_string(),
                "".to_string(),
            )
        })?;

        let from_address_tokens =
            ethabi::decode(&[ParamType::Address], from_address_bytes.as_bytes())
                .map_err(|e| {
                    ProcessorError::DecodeError(
                        "Failed to decode 'from' address".to_string(),
                        e.to_string(),
                    )
                })?;

        let from_address = from_address_tokens.first().ok_or_else(|| {
            ProcessorError::ValidationError(
                "Missing 'from' address in tokens".to_string(),
            )
        })?;

        let from = format!(
            "0x{}",
            hex::encode(from_address.clone().into_address().ok_or_else(|| {
                ProcessorError::ParseError(
                    "Failed to convert from_address into string".to_string(),
                    "".to_string(),
                )
            })?)
        );

        let to_address_bytes = array_bytes::hex_n_into::<String, H256, 32>(
            event.topic3.clone().ok_or_else(|| {
                ProcessorError::ValidationError("Missing 'to' address topic".to_string())
            })?,
        )
        .map_err(|_| {
            ProcessorError::ParseError(
                "Failed to parse 'to' address".to_string(),
                "".to_string(),
            )
        })?;

        let to_address_tokens =
            ethabi::decode(&[ParamType::Address], to_address_bytes.as_bytes()).map_err(
                |e| {
                    ProcessorError::DecodeError(
                        "Failed to decode 'to' address".to_string(),
                        e.to_string(),
                    )
                },
            )?;

        let to_address = to_address_tokens.first().ok_or_else(|| {
            ProcessorError::ValidationError("Missing 'to' address in tokens".to_string())
        })?;

        let to = format!(
            "0x{}",
            hex::encode(to_address.clone().into_address().ok_or_else(|| {
                ProcessorError::ParseError(
                    "Failed to convert to_address into string".to_string(),
                    "".to_string(),
                )
            })?)
        );

        let id = transfer_values[0].clone().into_uint().unwrap();
        let amount = transfer_values[1].clone().into_uint().unwrap();

        let transfer_data = Erc1155TransferData {
            contract_id,
            block_number: event.block_number as i32,
            chain_id: event.chain_id as i32,
            tx_hash: event.tx_hash.clone(),
            tx_index: event.tx_index,
            from,
            to,
            token_ids: [id.clone().to_string()].to_vec(),
            amounts: [amount.clone().to_string()].to_vec(),
        };

        self.erc1155_repository
            .insert_transfer(transfer_data)
            .await
            .map_err(|e| ProcessorError::DatabaseError(e.to_string()))?;

        Ok(ProcessResult::Stored)
    }
}