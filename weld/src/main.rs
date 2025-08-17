use clap::{Parser, Subcommand};
use eyre::Result;
use serde_json::from_str;
use token::get_tokens_info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get information about ERC20 tokens
    TokensInfo {
        /// Token Addresses (JSON array format) e.g. '["0x123..", "0x456.."]'
        #[arg(short = 'a', long)]
        addresses: String,

        /// RPC url e.g. 'https://rpc.ankr.com/eth'
        #[arg(short = 'r', long)]
        rpc_url: String,

        /// Write token information to file
        #[arg(short = 'w')]
        is_write: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::TokensInfo {
            addresses,
            rpc_url,
            is_write,
        } => {
            let addresses: Vec<String> = from_str(&addresses)?;
            get_tokens_info(addresses, rpc_url, is_write).await?;
        }
    }

    Ok(())
}
