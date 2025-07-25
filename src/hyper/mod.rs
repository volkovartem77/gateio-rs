//! Gate.io asynchronous HTTP client using Hyper.
//!
//! This module provides an async HTTP client implementation for the Gate.io API
//! using the [`hyper`] library. It's designed for high-performance applications
//! that need non-blocking I/O operations.
//!
//! # Features
//!
//! To use this async client, enable the `enable-hyper` feature:
//!
//! ```toml
//! [dependencies]
//! gateio-rs = { version = "0.1", features = ["enable-hyper"], default-features = false }
//! ```
//!
//! # Example
//!
//! ```no_run
//! use gateio_rs::{
//!     api::spot::get_ticker,
//!     http::Credentials,
//!     hyper::{GateHttpClient, Error},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // Set up credentials for authenticated requests
//!     let credentials = Credentials::new("your_api_key".to_string(), "your_api_secret".to_string());
//!     let client = GateHttpClient::default().credentials(credentials);
//!     
//!     // Get ticker data for BTC_USDT
//!     let request = get_ticker().currency_pair("BTC_USDT");
//!     let response = client.send(request).await?;
//!     let data = response.into_body_str().await?;
//!     println!("Ticker data: {}", data);
//!     
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod response;

pub use client::*;
pub use error::*;
pub use response::*;
