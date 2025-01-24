use alloy_primitives::TxHash;
use cowprotocol_client::client::{OrderBookApiClient, OrderBookApiConfig};
use eyre::Result;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let client = OrderBookApiClient::new(OrderBookApiConfig::default());
    let tx_hash = TxHash::from_str("0xd0405ae34a75f394ec20493988674ac8912787edbfdcd673b59239bd46ac1684")?;
    let solver_competition_latest = client.solver_competition_by_tx_hash(&tx_hash).await?;
    println!("{:#?}", solver_competition_latest);

    Ok(())
}
