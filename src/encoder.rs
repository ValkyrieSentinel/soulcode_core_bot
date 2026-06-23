use alloy::primitives::{Address, U256};
use std::str::FromStr;

pub struct AirdropEncoder;

impl AirdropEncoder {
    pub fn encode_airdrop(addresses: Vec<String>, amounts: Vec<String>) -> Result<String, String> {
        let length = addresses.len();
        
        if length == 0 || length != amounts.len() {
            return Err("Invalid input length".to_string());
        }

        let mut calldata = Vec::with_capacity(32 + length * 64);

        calldata.extend_from_slice(&U256::from(length).to_be_bytes::<32>());

        for i in 0..length {
            let addr = Address::from_str(addresses[i].trim())
                .map_err(|_| format!("Invalid address at index {}", i))?;
            
            let amount = U256::from_str(amounts[i].trim())
                .map_err(|_| format!("Invalid amount at index {}", i))?;

            let mut padded_addr = [0u8; 32];
            padded_addr[12..32].copy_from_slice(addr.as_slice());
            calldata.extend_from_slice(&padded_addr);

            calldata.extend_from_slice(&amount.to_be_bytes::<32>());
        }

        Ok(format!("0x{}", alloy::hex::encode(calldata)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soulcode_encoder_layout() {
        let addresses = vec![
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
            "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC".to_string(),
        ];
        let amounts = vec![
            "1000000000000000000".to_string(), // 1 ether
            "2000000000000000000".to_string(), // 2 ether
        ];

        let result = AirdropEncoder::encode_airdrop(addresses, amounts);
        assert!(result.is_ok());
        
        let hex = result.unwrap();
        // Перевіряємо довжину: перші 32 байти (64 символи) мають дорівнювати кількості адрес (2)
        assert!(hex.starts_with("0x0000000000000000000000000000000000000000000000000000000000000002"));
        println!("Rust-енкодер згенерував правильний плоский макет!");
    }
}
