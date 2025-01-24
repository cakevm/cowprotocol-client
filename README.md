# CoW Protocol REST API client

This crate provides an async client for interacting with the [CoW Protocol REST API](https://api.cow.fi/docs/#/). The benefit is to use typed models as they have been intended by the [Cow Protocol service](https://github.com/cowprotocol/services).

## Status
The crate is currently just a showcase and not for productive use. Generated clients can be difficult to use when dealing with byte types like U256, Address, etc. This client directly uses some models from the [services](https://github.com/cowprotocol/services) repository, ensuring compatibility with the API. The long-term goal is to migrate everything to use [Alloy](https://github.com/alloy-rs/alloy) types or have some converter. This crate currently supports a limited set of API endpoints. If you need additional endpoints, please open an issue or a PR.

Current concept for types:
- The functions from this client accepting [alloy_primitives](https://docs.rs/alloy-primitives/latest/alloy_primitives/)
- The return types can be models from [services](https://github.com/cowprotocol/services) with [ethereum-types](https://crates.io/crates/ethereum-types) or like Auction from this crate.

Future concept for types:
- Alloy for all types

### APIs
```
- Solver API (driver)
  - /quote (GET)
  - /solve (POST)
  - /reveal (POST)
  - /settle (POST)

- Solver Engine API (solvers)
  - /solve (POST)
  - /notify (POST)

- Order Book API (orderbook)
  - /orders (POST, DELETE)
  - /orders/{UID} (GET, DELETE)
  - /orders/{UID}/status (GET)
  - /transactions/{txHash}/orders (GET)
  - /trades (GET)
  - /auction (GET)
  - /account/{owner}/orders (GET)
  - /token/{token}/native_price (GET)
  - /quote (POST)
  - /solver_competition/{auction_id} (GET)
  - /solver_competition/by_tx_hash/{tx_hash} (GET)
  - /solver_competition/latest (GET)
  - /version (GET)
  - /app_data/{app_data_hash} (GET, PUT)
  - /app_data (PUT)
  - /users/{address}/total_surplus (GET)
```

## Usage
See the [examples](./examples) directory for usage examples.

## Acknowledgements
Many thanks to the [CoW Protocol](https://github.com/cowprotocol) team for making their service open source. This project uses the models from the [services](https://github.com/cowprotocol/services) repository.

## License
This project is dual licensed under [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT). 