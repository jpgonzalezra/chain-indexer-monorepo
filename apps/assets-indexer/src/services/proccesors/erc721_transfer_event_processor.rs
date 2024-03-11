use async_trait::async_trait;
use common::types::SummaryLog;

use super::event_processor::EventProcessor;

pub struct Erc721TransferProcessor;

#[async_trait]
impl EventProcessor for Erc721TransferProcessor {
    async fn process(&self, log: &SummaryLog) -> bool {
        true
    }
}
