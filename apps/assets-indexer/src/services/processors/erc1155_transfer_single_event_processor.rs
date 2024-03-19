// use async_trait::async_trait;

// use super::event_processor::{
//     EventProcessor, EventProcessorRequest, ProcessResult, ProcessorError,
// };
// use ethers::{
//     abi::{ethabi, ParamType},
//     utils::hex,
// };
// pub struct Erc1155TransferSingleProcessor;

// impl Erc1155TransferSingleProcessor {
//     const ERC1155_TRANSFER_SINGLE_EVENT_SIGNATURE: &'static str =
//         "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
// }
// #[async_trait]
// impl EventProcessor for Erc1155TransferSingleProcessor {
//     async fn store_if_apply(
//         &self,
//         event: &EventProcessorRequest,
//     ) -> Result<ProcessResult, ProcessorError> {
//         if (event.topic0.to_lowercase() != Self::ERC1155_TRANSFER_SINGLE_EVENT_SIGNATURE)
//             || (event.topic1.is_none()
//                 || event.topic2.is_none()
//                 || event.topic3.is_none())
//         {
//             return Ok(ProcessResult::NotApplicable);
//         }

//         let data = hex::decode(&event.data[2..]).map_err(|e| {
//             ProcessorError::DecodeError(e.to_string(), event.data.clone())
//         })?;

//         let transfer_values =
//             ethabi::decode(&[ParamType::Uint(256), ParamType::Uint(256)], &data[..])
//                 .map_err(|e| {
//                     ProcessorError::DecodeError(e.to_string(), event.data.clone())
//                 })?;

//         let id = transfer_values[0].clone().into_uint().unwrap();
//         let amount = transfer_values[1].clone().into_uint().unwrap();
//         println!("{:?}, {:?}", id, amount);
//         Ok(ProcessResult::Stored);
//     }
// }
