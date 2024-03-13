use async_trait::async_trait;

#[async_trait]
pub trait EventProcessor {
    async fn process(&self, event: &EventProcessorRequest) -> bool;
}

pub struct EventProcessorRequest {
    pub address: String,
    pub data: String,
    pub topic0: String,
    pub topic1: Option<String>,
    pub topic2: Option<String>,
    pub topic3: Option<String>,
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

    pub async fn process(&self, event: &EventProcessorRequest) -> bool {
        for processor in &self.processors {
            if processor.process(event).await {
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
