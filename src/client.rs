use crate::types::{ApiUrl, Chain, OrderBookApiError};
use crate::types::{auction::AuctionWithId};
use alloy_primitives::TxHash;
use cowprotocol_solvers_dto_alloy::order_uid::OrderUid;
use reqwest::{Client, ClientBuilder};
use services_model::order::Order;
use services_model::solver_competition::SolverCompetitionAPI;

#[derive(Debug, Clone)]
pub struct OrderBookApiClient {
    client: Client,
    url: ApiUrl,
}

#[derive(Debug, Clone, Default)]
pub struct OrderBookApiConfig {
    pub chain: Chain,
}

impl OrderBookApiClient {
    /// Create a new client with the given configuration.
    pub fn new(cfg: OrderBookApiConfig) -> Self {
        let builder = ClientBuilder::new();
        let client = builder.build().unwrap();
        let base_url = format!("{}/api/v1", cfg.chain.url());

        Self { client, url: ApiUrl { base: base_url } }
    }

    pub async fn auction(&self) -> Result<AuctionWithId, OrderBookApiError> {
        let res = self.client.get(self.url.auction()).send().await?.error_for_status()?.json::<AuctionWithId>().await?;
        Ok(res)
    }

    pub async fn solver_competition_latest(&self) -> Result<SolverCompetitionAPI, OrderBookApiError> {
        let res =
            self.client.get(self.url.solver_competition_latest()).send().await?.error_for_status()?.json::<SolverCompetitionAPI>().await?;
        Ok(res)
    }

    pub async fn solver_competition_by_tx_hash(&self, tx_hash: &TxHash) -> Result<SolverCompetitionAPI, OrderBookApiError> {
        let res = self
            .client
            .get(self.url.solver_competition_by_tx_hash(tx_hash))
            .send()
            .await?
            .error_for_status()?
            .json::<SolverCompetitionAPI>()
            .await?;
        Ok(res)
    }

    pub async fn get_order(&self, order_uid: &OrderUid) -> Result<Order, OrderBookApiError> {
        let res = self.client.get(self.url.get_order(order_uid)).send().await?.error_for_status()?.json::<Order>().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, hex, U256};
    use ethereum_types::{H160, U256 as EthereumU256};
    use std::path::PathBuf;

    #[test]
    fn can_deserialize_auction_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_auction.json");

        let res = std::fs::read_to_string(d).unwrap();
        let res: AuctionWithId = serde_json::from_str(&res).unwrap();
        assert_eq!(res.id, 9008466);
        assert_eq!(*res.prices.get(&address!("0001a500a6b18995b03f44bb040a5ffc28e45cb0")).unwrap(), U256::from(465582784046718u64))
    }

    #[test]
    fn can_deserialize_solver_competition_latest_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_solver_competition_latest.json");

        let res = std::fs::read_to_string(d).unwrap();
        let res: SolverCompetitionAPI = serde_json::from_str(&res).unwrap();
        assert_eq!(res.auction_id, 9008649);
        assert_eq!(
            res.common.auction.prices.get(&H160(hex!("0001a500a6b18995b03f44bb040a5ffc28e45cb0"))),
            Some(&EthereumU256::from(464799964606399u64))
        )
    }
}
