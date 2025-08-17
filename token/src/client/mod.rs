use alloy::{primitives::Address, providers::ProviderBuilder, sol};
use alloy_provider::Provider;
use eyre::Result;

use crate::types::TokenInfo;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "src/abi/IERC20.json"
);

pub struct TokenClient {
    rpc_url: String,
}

impl TokenClient {
    pub fn new(rpc_url: String) -> Self {
        TokenClient { rpc_url }
    }

    pub async fn get_token_info(&self, token_address: Address) -> Result<TokenInfo> {
        // Create provider for this request
        let rpc_url = self.rpc_url.parse()?;
        let provider = ProviderBuilder::new().connect_http(rpc_url);

        // Create a contract instance
        let erc20 = IERC20::new(token_address, &provider);

        // Multicall to get all token info at once
        let multicall = provider
            .multicall()
            .add(erc20.name())
            .add(erc20.symbol())
            .add(erc20.decimals());

        let (name, symbol, decimals) = multicall.aggregate().await?;

        Ok(TokenInfo {
            address: token_address,
            name,
            symbol,
            decimals,
        })
    }
}
