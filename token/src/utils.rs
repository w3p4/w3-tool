use alloy::primitives::Address;
use eyre::Result;

pub fn parse_addresses(addresses: Vec<String>) -> Result<Vec<Address>> {
    let parsed_addresses = addresses
        .into_iter()
        .map(|addr| addr.parse::<Address>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(parsed_addresses)
}
