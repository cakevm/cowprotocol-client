use cowprotocol_api_rs::client::{OrderBookApiClient, OrderBookApiConfig};

#[tokio::main]
async fn main() {
    let client = OrderBookApiClient::new(OrderBookApiConfig::default());
    let auction_result = client.auction().await;
    match auction_result {
        Ok(auction) => {
            println!("Auction ID: {}", auction.id);
            match auction.orders.first() {
                Some(order) => println!("First Order: {:#?}", order),
                None => println!("No orders found."),
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
