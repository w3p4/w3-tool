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

const FILE_NAME: &str = "token_infos.json";

pub async fn get_tokens_info(
    addresses: Vec<String>,
    rpc_url: String,
    is_write_file: bool,
) -> Result<()> {
    let parsed_addresses = parse_addresses(addresses)?;

    let mut token_service = TokenService::new(rpc_url);
    token_service.get_tokens_info(parsed_addresses).await?;
    token_service.print_token_infos();

    if is_write_file {
        token_service.write_to_file(FILE_NAME);
    }

    Ok(())
}
