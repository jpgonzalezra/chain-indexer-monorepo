use async_trait::async_trait;
use ethers::{
    abi::{ethabi, ParamType},
    types::U256,
    utils::hex,
};

use super::event_processor::{EventProcessor, EventProcessorRequest};

pub struct Erc1155TransferBatchProcessor;

impl Erc1155TransferBatchProcessor {
    const ERC1155_TRANSFER_BATCH_EVENT_SIGNATURE: &'static str =
        "0x4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb";
}

#[async_trait]
impl EventProcessor for Erc1155TransferBatchProcessor {
    async fn process(&self, event: &EventProcessorRequest) -> bool {
        if event.topic0.to_lowercase() != Self::ERC1155_TRANSFER_BATCH_EVENT_SIGNATURE {
            return false;
        }
        if event.topic1.is_none() || event.topic2.is_none() || event.topic3.is_none() {
            return false;
        }

        let data = hex::decode(&event.data[2..]).unwrap();

        let transfer_values = ethabi::decode(
            &[
                ParamType::Array(Box::new(ParamType::Uint(256))),
                ParamType::Array(Box::new(ParamType::Uint(256))),
            ],
            &data[..],
        )
        .unwrap();

        let ids: Vec<U256> = transfer_values[0]
            .clone()
            .into_array()
            .unwrap()
            .iter()
            .map(|token| token.clone().into_uint().unwrap())
            .collect();
        let amounts: Vec<U256> = transfer_values[1]
            .clone()
            .into_array()
            .unwrap()
            .iter()
            .map(|token| token.clone().into_uint().unwrap())
            .collect();
        println!("{:?}, {:?}", ids, amounts);

        return true;
    }
}
