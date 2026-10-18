use alloy::primitives::{Address, U256};
use std::str::FromStr;

pub struct AirdropEncoder;
impl AirdropEncoder {
    const DISPERSE_SELECTOR: [u8; 4] = [0x00, 0x00, 0x00, 0x00]; // TODO: keccak256("disperse()")[0..4]
    const MAX_BATCH_SIZE: usize = 500;
    pub fn encode_airdrop(addresses: Vec<String>, amounts: Vec<String>) -> Result<String, String> {
        let length = addresses.len();
        if length == 0 || length != amounts.len() { return Err("Length mismatch".to_string()); }
        if length > Self::MAX_BATCH_SIZE { return Err("Batch too large".to_string()); }
        let mut calldata = Vec::with_capacity(4 + 32 + length * 64);
        calldata.extend_from_slice(&Self::DISPERSE_SELECTOR);
        calldata.extend_from_slice(&U256::from(length).to_be_bytes::<32>());
        for i in 0..length {
            let addr = Address::from_str(addresses[i].trim()).map_err(|_| format!("Invalid addr at {}", i))?;
            let amount = U256::from_str(amounts[i].trim()).map_err(|_| format!("Invalid amount at {}", i))?;
            let mut padded = [0u8; 32]; padded[12..32].copy_from_slice(addr.as_slice());
            calldata.extend_from_slice(&padded);
            calldata.extend_from_slice(&amount.to_be_bytes::<32>());
        }
        Ok(format!("0x{}", alloy::hex::encode(calldata)))
    }
}