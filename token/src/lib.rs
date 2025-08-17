pub mod client;
pub mod service;
pub mod types;
pub mod utils;

use eyre::Result;
use service::TokenService;

pub use client::TokenClient;
pub use service::TokenService as Service;
pub use types::TokenInfo;
pub use utils::parse_addresses;

pub struct TokenManager {
    service: TokenService,
}

impl TokenManager {
    pub fn new(rpc_url: String) -> Self {
        Self {
            service: TokenService::new(rpc_url),
        }
    }

    pub async fn get_tokens_info(&mut self, addresses: Vec<String>) -> Result<()> {
        let parsed_addresses = parse_addresses(addresses)?;
        self.service.get_tokens_info(parsed_addresses).await?;
        self.service.print_token_infos();
        Ok(())
    }

    pub fn write_to_file(&self, file_name: &str) -> Result<()> {
        self.service.write_to_file(file_name)?;
        Ok(())
    }
}
