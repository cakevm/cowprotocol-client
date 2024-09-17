pub mod auction;
pub mod solver_competition;

use crate::constants::{
    API_BASE_ARBITRUM_ONE_PROD, API_BASE_ARBITRUM_ONE_STAGING, API_BASE_GNOSIS_CHAIN_PROD, API_BASE_GNOSIS_CHAIN_STAGING,
    API_BASE_MAINNET_PROD, API_BASE_MAINNET_STAGING, API_BASE_SEPOLIA_PROD, API_BASE_SEPOLIA_STAGING,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrderBookApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Default)]
pub enum Chain {
    #[default]
    Ethereum,
    GnosisChain,
    Arbitrum,
    Sepolia,
    EthereumStaging,
    GnosisChainStaging,
    ArbitrumStaging,
    SepoliaStaging,
}

impl Chain {
    pub(crate) fn url(&self) -> &'static str {
        match self {
            Chain::Ethereum => API_BASE_MAINNET_PROD,
            Chain::GnosisChain => API_BASE_GNOSIS_CHAIN_PROD,
            Chain::Arbitrum => API_BASE_ARBITRUM_ONE_PROD,
            Chain::Sepolia => API_BASE_SEPOLIA_PROD,
            Chain::EthereumStaging => API_BASE_MAINNET_STAGING,
            Chain::GnosisChainStaging => API_BASE_GNOSIS_CHAIN_STAGING,
            Chain::ArbitrumStaging => API_BASE_ARBITRUM_ONE_STAGING,
            Chain::SepoliaStaging => API_BASE_SEPOLIA_STAGING,
        }
    }
}

/// API endpoints
#[derive(Debug, Clone)]
pub struct ApiUrl {
    pub base: String,
}

impl ApiUrl {
    pub fn auction(&self) -> String {
        format!("{}/auction", self.base)
    }

    pub fn solver_competition_latest(&self) -> String {
        format!("{}/solver_competition/latest", self.base)
    }
}
