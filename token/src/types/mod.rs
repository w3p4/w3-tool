use alloy::primitives::Address;

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}
