use alloy_primitives::{Address, Bytes, TxHash, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolverCompetition {
    pub auction_id: i64,
    pub transaction_hash: TxHash,
    pub auction_start_block: u64,
    pub competition_simulation_block: u64,
    pub auction: CompetitionAuction,
    pub solutions: Vec<SolverSettlement>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionAuction {
    pub orders: Vec<String>,
    pub prices: HashMap<Address, U256>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolverSettlement {
    pub solver: String,
    pub solver_address: Address,
    pub score: U256,
    pub ranking: usize,
    pub clearing_prices: HashMap<Address, U256>,
    pub orders: Vec<Order>,
    pub call_data: Option<Bytes>,
    pub uninternalized_call_data: Option<Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub sell_amount: String,
    pub buy_amount: String,
}
