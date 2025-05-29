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
    let token_address = Address::parse_checksummed(&args.address, None)?;
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = args.rpc_url;
    let provider =
        ProviderBuilder::new().connect_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;

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

    Ok(())
}
