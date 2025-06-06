use alloy::{primitives::Address, providers::ProviderBuilder, sol};
use alloy_provider::Provider;
use eyre::Result;
use serde_json::{json, to_string_pretty};
use std::fs::File;
use std::io::Write;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "src/abi/IERC20.json"
);

pub struct TokenInfo {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

pub async fn get_token_info(
    addresses: Vec<String>,
    rpc_url: String,
    output_json: bool,
) -> Result<()> {
    // Parse addresses
    let addresses: Vec<Address> = addresses
        .into_iter()
        .map(|addr| addr.parse::<Address>())
        .collect::<Result<Vec<_>, _>>()?;

    // Setup provider
    let rpc_url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let mut all_token_data = Vec::new();

    println!("----------- Tokens -------------\n");
    // Process each address
    for token_address in addresses {
        // Create a contract instance.
        let erc20 = IERC20::new(token_address, &provider);

        // Multicall
        let multicall = provider
            .multicall()
            .add(erc20.name())
            .add(erc20.symbol())
            .add(erc20.decimals());
        let (name, symbol, decimals) = multicall.aggregate().await?;
        println!("{name}, {symbol}, {decimals}");

        // Add to token data collection
        all_token_data.push(json!({
            "address": token_address.to_string(),
            "name": name,
            "symbol": symbol,
            "decimals": decimals
        }));
    }
    println!("\n--------------------------------");

    // Write to JSON file if output_json is true
    if output_json {
        let json_string = to_string_pretty(&all_token_data)?;
        let mut file = File::create("token_data.json")?;
        file.write_all(json_string.as_bytes())?;
        println!("\nToken data written to token_data.json");
    }

    Ok(())
} 