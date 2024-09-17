use crate::{
    constants::PROTOCOL_VERSION,
    types::{ApiUrl, Chain, OrderBookApiError},
};

use crate::types::{auction::AuctionWithId, solver_competition::SolverCompetition};
use reqwest::{Client, ClientBuilder};

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
        let base_url = format!("{}/api/{}", cfg.chain.url(), PROTOCOL_VERSION);

        Self { client, url: ApiUrl { base: base_url } }
    }

    pub async fn auction(&self) -> Result<AuctionWithId, OrderBookApiError> {
        let res = self.client.get(self.url.auction()).send().await?.json::<AuctionWithId>().await?;
        Ok(res)
    }

    pub async fn solver_competition_latest(&self) -> Result<SolverCompetition, OrderBookApiError> {
        let res = self.client.get(self.url.solver_competition_latest()).send().await?.json::<SolverCompetition>().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::solver_competition::SolverCompetition;
    use alloy_primitives::{address, U256};
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
        let res: SolverCompetition = serde_json::from_str(&res).unwrap();
        assert_eq!(res.auction_id, 9008649);
        assert_eq!(*res.auction.prices.get(&address!("0001a500a6b18995b03f44bb040a5ffc28e45cb0")).unwrap(), U256::from(464799964606399u64))
    }
}
