use async_trait::async_trait;
use common::types::SummaryLog;

#[async_trait]
pub trait EventProcessor {
    async fn process(&self, log: &SummaryLog) -> bool;
}

pub struct EventProcessorRequest {
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
}

pub struct EventProcessorService {
    processors: Vec<Box<dyn EventProcessor>>,
}

impl EventProcessorService {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn EventProcessor>) {
        self.processors.push(processor);
    }

    pub async fn process(&self, log: &SummaryLog) -> bool {
        for processor in &self.processors {
            if processor.process(log).await {
                return true;
            }
        }
        false
    }
}

impl Default for EventProcessorService {
    fn default() -> Self {
        Self::new()
    }
}
