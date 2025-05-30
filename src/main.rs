//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

use alloy::{primitives::Address, providers::ProviderBuilder, sol};
use alloy_provider::Provider;
use eyre::Result;
use clap::Parser;
use serde_json::{json, to_string_pretty, from_str};
use std::fs::File;
use std::io::Write;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "src/abi/IERC20.json"
);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Token Addresses (JSON array format) e.g. '["0x123..", "0x456.."]'
    #[arg(short = 'a', long)]
    addresses: String,

    /// RPC url e.g. 'https://rpc.ankr.com/eth'
    #[arg(short = 'r', long)]
    rpc_url: String,

    /// Output as JSON file
    #[arg(short = 'j', long)]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Parse addresses from JSON array
    let addresses: Vec<String> = from_str(&args.addresses)?;
    let addresses: Vec<Address> = addresses
        .into_iter()
        .map(|addr| Address::parse_checksummed(&addr, None))
        .collect::<Result<Vec<_>, _>>()?;

    // define rpc url
    let rpc_url = args.rpc_url.parse()?;
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
    println!("\n--------------------------------\n");

    // Write to JSON file if --json flag is set
    if args.json {
        let json_string = to_string_pretty(&all_token_data)?;
        let mut file = File::create("token_data.json")?;
        file.write_all(json_string.as_bytes())?;
        println!("Token data written to token_data.json");
    }

    Ok(())
}
