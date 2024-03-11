use async_trait::async_trait;
use common::types::SummaryLog;

use super::event_processor::EventProcessor;

pub struct Erc1155TransferProcessor;

#[async_trait]
impl EventProcessor for Erc1155TransferProcessor {
    async fn process(&self, log: &SummaryLog) -> bool {
        true
    }
}
