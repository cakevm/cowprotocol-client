use cowprotocol_client::client::{OrderBookApiClient, OrderBookApiConfig};

#[tokio::main]
async fn main() {
    let client = OrderBookApiClient::new(OrderBookApiConfig::default());
    let solver_competition_latest = client.solver_competition_latest().await;
    match solver_competition_latest {
        Ok(solver_competition) => {
            println!("Auction ID: {}", solver_competition.auction_id);
            match solver_competition.common.solutions.first() {
                Some(solution) => {
                    println!("First Solution: {:#?}", solution);
                }
                None => {
                    println!("No solution found.");
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
