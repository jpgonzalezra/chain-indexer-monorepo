use async_trait::async_trait;
use ethers::{
    abi::{ethabi, ParamType},
    utils::hex,
};

use super::event_processor::{EventProcessor, EventProcessorRequest};

pub struct Erc721TransferProcessor;

impl Erc721TransferProcessor {
    const TRANSFER_TOPIC: &'static str =
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
}

#[async_trait]
impl EventProcessor for Erc721TransferProcessor {
    async fn process(&self, event: &EventProcessorRequest) -> bool {
        if event.topic0.to_lowercase() != Self::TRANSFER_TOPIC {
            return false;
        }
        if event.topic1.is_none() || event.topic2.is_none() || event.topic3.is_none() {
            return false;
        }

        let data = hex::decode(&event.data[2..]).unwrap();

        let transfer_data =
            ethabi::decode(&[ParamType::Uint(256), ParamType::Uint(256)], &data[..])
                .unwrap();

        let id = transfer_data[0].clone().into_uint().unwrap();
        let amount = transfer_data[1].clone().into_uint().unwrap();
        println!("{:?}, {:?}", id, amount);

        return true;
    }
}
