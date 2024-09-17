use alloy_primitives::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionWithId {
    pub id: i64,
    pub block: u64,
    pub latest_settlement_block: u64,
    pub orders: Vec<Order>,
    pub prices: HashMap<Address, U256>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderKind {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
    Market,
    Liquidity,
    Limit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SellTokenSource {
    Erc20,
    External,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuyTokenDestination {
    Erc20,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SigningScheme {
    Eip712,
    EthSign,
    Eip1271,
    PreSign,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub uid: Bytes,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_amount: U256,
    pub buy_amount: U256,
    pub protocol_fees: Vec<ProtocolFee>,
    pub valid_to: u32,
    pub kind: OrderKind,
    pub receiver: Address,
    pub owner: Address,
    pub partially_fillable: bool,
    pub executed: String,
    pub pre_interactions: Vec<PreInteraction>,
    pub post_interactions: Vec<Value>,
    pub sell_token_balance: SellTokenSource,
    pub buy_token_balance: BuyTokenDestination,
    pub class: OrderClass,
    pub app_data: Bytes,
    pub signing_scheme: SigningScheme,
    pub signature: Bytes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolFee {
    pub price_improvement: Option<PriceImprovement>,
    pub surplus: Option<Surplus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceImprovement {
    pub factor: f64,
    pub max_volume_factor: f64,
    pub quote: Quote,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub sell_amount: U256,
    pub buy_amount: U256,
    pub fee: U256,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Surplus {
    pub factor: f64,
    pub max_volume_factor: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreInteraction {
    pub target: Address,
    pub value: U256,
    pub call_data: Bytes,
}
