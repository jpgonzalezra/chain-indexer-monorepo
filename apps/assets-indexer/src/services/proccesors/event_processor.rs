use std::fmt;

use async_trait::async_trait;

#[derive(Debug)]
pub enum ProcessorError {
    ParseError(String, String),
    DecodeError(String, String),
    ValidationError(String),
    DatabaseError(String),
}

pub enum ProcessResult {
    Stored,
    NotApplicable,
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorError::ParseError(error, data) => {
                write!(f, "Parse Error: {}. Data: {}", error, data)
            }
            ProcessorError::DecodeError(error, data) => {
                write!(f, "Decode Error: {}. Data: {}", error, data)
            }
            ProcessorError::ValidationError(ref err) => {
                write!(f, "Validation Error: {}", err)
            }
            ProcessorError::DatabaseError(ref err) => {
                write!(f, "Database Error: {}", err)
            }
        }
    }
}

#[async_trait]
pub trait EventProcessor {
    async fn store_if_apply(
        &self,
        event: &EventProcessorRequest,
    ) -> Result<ProcessResult, ProcessorError>;
}

#[derive(Debug)]
pub struct EventProcessorRequest {
    pub address: String,
    pub data: String,
    pub chain_id: u32,
    pub block_number: u64,
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

    pub async fn process_and_store_if_apply(&self, event: &EventProcessorRequest) {
        for processor in &self.processors {
            match processor.store_if_apply(event).await {
                Ok(ProcessResult::Stored) => {
                    println!("Event stored successfully {:?}", event);
                }
                Ok(ProcessResult::NotApplicable) => {
                    // no-op
                }
                Err(e) => {
                    eprintln!("Error processing event: {:?}", e);
                }
            }
        }
    }
}

impl Default for EventProcessorService {
    fn default() -> Self {
        Self::new()
    }
}
