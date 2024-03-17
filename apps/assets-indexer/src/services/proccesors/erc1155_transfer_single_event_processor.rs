use async_trait::async_trait;

use super::event_processor::{EventProcessor, EventProcessorRequest};
use ethers::{
    abi::{ethabi, ParamType},
    utils::hex,
};
pub struct Erc1155TransferSingleProcessor;

impl Erc1155TransferSingleProcessor {
    const ERC1155_TRANSFER_SINGLE_EVENT_SIGNATURE: &'static str =
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
}
#[async_trait]
impl EventProcessor for Erc1155TransferSingleProcessor {
    async fn process(&self, event: &EventProcessorRequest) -> bool {
        if event.topic0.to_lowercase() != Self::ERC1155_TRANSFER_SINGLE_EVENT_SIGNATURE {
            return false;
        }
        if event.topic1.is_none() || event.topic2.is_none() || event.topic3.is_none() {
            return false;
        }

        let data = match hex::decode(&event.data[2..]) {
            Ok(decoded) => decoded,
            Err(e) => {
                eprintln!("Error decoding event data: {:?}", e);
                Vec::new()
            }
        };

        if data.is_empty() {
            return false;
        }

        let transfer_values = match ethabi::decode(
            &[ParamType::Uint(256), ParamType::Uint(256)],
            &data[..],
        ) {
            Ok(decoded) => decoded,
            Err(e) => {
                eprintln!(
                    "Error decoding ABI data: {:?}, error message: {:?}",
                    data, e
                );
                Vec::new()
            }
        };

        if transfer_values.is_empty() {
            return false;
        }

        let id = transfer_values[0].clone().into_uint().unwrap();
        let amount = transfer_values[1].clone().into_uint().unwrap();
        println!("{:?}, {:?}", id, amount);
        return true;
    }
}
