use alloy::primitives::Address;
use eyre::Result;
use serde_json::{json, to_string_pretty};
use std::fs::File;
use std::io::Write;

use crate::client::TokenClient;
use crate::types::TokenInfo;

pub struct TokenService {
    client: TokenClient,
    token_infos: Vec<TokenInfo>,
}

impl TokenService {
    pub fn new(rpc_url: String) -> Self {
        TokenService {
            client: TokenClient::new(rpc_url),
            token_infos: Vec::new(),
        }
    }

    pub async fn get_tokens_info(&mut self, addresses: Vec<Address>) -> Result<()> {
        self.token_infos.clear();

        for address in addresses {
            let token_info = self.client.get_token_info(address).await?;
            self.token_infos.push(token_info);
        }

        Ok(())
    }

    pub fn convert_to_json_string(&self) -> Result<String> {
        let json_data: Vec<_> = self
            .token_infos
            .iter()
            .map(|token_info| {
                json!({
                    "address": token_info.address.to_string(),
                    "name": token_info.name,
                    "symbol": token_info.symbol,
                    "decimals": token_info.decimals
                })
            })
            .collect();
        let json_string = to_string_pretty(&json_data)?;
        Ok(json_string)
    }

    pub fn write_to_file(&self, file_name: &str) -> Result<()> {
        let json_string = self.convert_to_json_string()?;
        let mut file = File::create(file_name).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
        Ok(())
    }

    pub fn print_token_infos(&self) {
        println!("----------- Tokens -------------\n");
        for token_info in self.token_infos.iter() {
            println!(
                "name: {}, symbol: {}, decimals: {}",
                token_info.name, token_info.symbol, token_info.decimals
            );
        }
        println!("\n--------------------------------");
    }
}
