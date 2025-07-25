# Gate.io Rust SDK

A comprehensive Rust SDK for the [Gate.io](https://www.gate.com) cryptocurrency exchange API, supporting both synchronous and asynchronous HTTP clients.

## Features

- **Dual client support**: Synchronous (ureq) and asynchronous (hyper) HTTP clients
- **Complete Spot API**: All Gate.io Spot trading endpoints implemented
- **Type safety**: Strong typing for all API parameters and responses
- **Authentication**: Automatic HMAC SHA-512 signing for authenticated requests
- **Builder pattern**: Ergonomic request building with optional parameters

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
# For synchronous client (default)
[dependencies]
gateio-rs = "0.1"

# For asynchronous client
[dependencies]
gateio-rs = { version = "0.1", features = ["enable-hyper"], default-features = false }
```

### Basic Usage

#### Public API (No Authentication Required)

```rust
use gateio_rs::{api::spot::get_ticker, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GateHttpClient::default();
    
    // Get ticker for BTC_USDT
    let request = get_ticker()
        .currency_pair("BTC_USDT")
        .timezone("utc8");
    
    let response = client.send(request)?;
    let body = response.into_body_str()?;
    let ticker_data: Value = serde_json::from_str(&body)?;
    
    println!("BTC_USDT Ticker: {}", serde_json::to_string_pretty(&ticker_data)?);
    Ok(())
}
```

#### Authenticated API

```rust
use gateio_rs::{
    api::spot::get_account,
    http::Credentials,
    ureq::GateHttpClient,
};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up credentials
    let credentials = Credentials::new("your_api_key", "your_api_secret");
    let client = GateHttpClient::default().credentials(credentials);
    
    // Get account information
    let request = get_account();
    let response = client.send(request)?;
    let body = response.into_body_str()?;
    let account_data: Value = serde_json::from_str(&body)?;
    
    println!("Account: {}", serde_json::to_string_pretty(&account_data)?);
    Ok(())
}
```

## API Coverage

The SDK provides complete coverage of Gate.io Spot trading API endpoints:

- **Market Data**: Tickers, order books, trades, candlesticks
- **Account Management**: Balance, trading fees, account history
- **Trading**: Create/cancel orders, batch operations, order history
- **Currency Information**: Supported currencies and trading pairs

## Documentation

- [Gate.io API Documentation](https://www.gate.com/docs/developers/apiv4/#spot)
- [Examples](examples/sync/) - Complete working examples for all endpoints

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.