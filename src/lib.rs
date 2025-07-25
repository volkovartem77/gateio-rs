//! # Gate.io Rust SDK
//!
//! A comprehensive Rust SDK for the Gate.io cryptocurrency exchange API, supporting both synchronous and asynchronous HTTP clients.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gateio_rs::{
//!     api::spot::get_ticker,
//!     http::Credentials,
//!     ureq::GateHttpClient,
//! };
//!
//! // Create client with credentials
//! let credentials = Credentials::new("your_api_key".to_string(), "your_api_secret".to_string());
//! let client = GateHttpClient::default().credentials(credentials);
//!
//! // Get ticker data
//! let request = get_ticker().currency_pair("BTC_USDT");
//! let response = client.send(request)?;
//! # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
//! ```
//!
//! ## Features
//!
//! This SDK provides dual client implementations controlled by feature flags:
//!
//! * `enable-ureq` (default): Synchronous HTTP client powered by [`ureq`](https://docs.rs/ureq/)
//! * `enable-hyper`: Asynchronous HTTP client powered by [`hyper`](https://docs.rs/hyper/)
//!
//! ## Architecture
//!
//! - **Spot Trading API**: Complete implementation of Gate.io Spot trading endpoints
//! - **Authentication**: Automatic HMAC SHA-512 signing for authenticated requests
//! - **Builder Pattern**: Ergonomic request building with optional parameters
//! - **Type Safety**: Strong typing for all API parameters and responses
//!

#![warn(missing_docs)]

mod utils;
mod version;

pub mod api;
pub mod http;

#[cfg(feature = "enable-hyper")]
pub mod hyper;

#[cfg(feature = "enable-ureq")]
pub mod ureq;
