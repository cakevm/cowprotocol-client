use alloy_primitives::FixedBytes;
use cowprotocol_client::client::{OrderBookApiClient, OrderBookApiConfig};
use cowprotocol_solvers_dto_alloy::order_uid::OrderUid;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let client = OrderBookApiClient::new(OrderBookApiConfig::default());
    let order_uid = OrderUid(
        FixedBytes::<56>::from_str(
            "0x1e1e4b3c27038cfe9b63c7c662ece8e722667f0b4316d502ee29dad6c49392c0891a539d008f69e62f22902877cce54a58644cae67875b9f",
        )
        .unwrap(),
    );
    match client.get_order(&order_uid).await {
        Ok(order) => {
            println!("{:?}", order);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
