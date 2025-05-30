//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

use alloy::{primitives::Address, providers::ProviderBuilder, sol};
use alloy_provider::Provider;
use eyre::Result;
use clap::Parser;

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
    // Token Address
    #[arg(short, long)]
    address: String,

    /// RPC url
    #[arg(short, long)]
    rpc_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    // TODO: accept many addresses
    let token_address = Address::parse_checksummed(&args.address, None)?;
    // define rpc url
    let rpc_url = args.rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // TODO: loop through all addresses
    // Create a contract instance.
    let erc20 = IERC20::new(token_address, &provider);

    // Multicall to get the token info
    let multicall = provider
        .multicall()
        .add(erc20.name())
        .add(erc20.symbol())
        .add(erc20.decimals());
    let (name, symbol, decimals) = multicall.aggregate().await?;
    println!("{name}, {symbol}, {decimals}, {token_address}");
    // TODO: write to json file

    Ok(())
}
