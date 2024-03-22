use super::event_processor::ProcessorError;
use ethers::{
    abi::{ethabi, ParamType},
    types::H256,
    utils::hex,
};

pub async fn decode_address(
    address_topic: Option<String>,
) -> Result<String, ProcessorError> {
    let address_bytes =
        array_bytes::hex_n_into::<String, H256, 32>(address_topic.ok_or_else(|| {
            ProcessorError::ValidationError("Missing address topic".to_string())
        })?)
        .map_err(|_| {
            ProcessorError::ParseError(
                "Failed to parse address".to_string(),
                "".to_string(),
            )
        })?;

    let address_tokens = ethabi::decode(&[ParamType::Address], address_bytes.as_bytes())
        .map_err(|e| {
            ProcessorError::DecodeError(
                "Failed to decode address".to_string(),
                e.to_string(),
            )
        })?;

    let address = address_tokens.first().ok_or_else(|| {
        ProcessorError::ValidationError("Missing address in tokens".to_string())
    })?;

    Ok(format!(
        "0x{}",
        hex::encode(address.clone().into_address().ok_or_else(|| {
            ProcessorError::ParseError(
                "Failed to convert address into string".to_string(),
                "".to_string(),
            )
        })?)
    ))
}
