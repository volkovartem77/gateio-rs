//! Gate.io synchronous HTTP client using ureq.
//!
//! This module provides a blocking HTTP client implementation for the Gate.io API
//! using the [`ureq`] library. This is the default client and is ideal for
//! synchronous applications and simple scripts.
//!
//! # Features
//!
//! This is the default client - no additional features needed:
//!
//! ```toml
//! [dependencies]
//! gateio-rs = "0.1"
//! ```
//!
//! # Usage
//!
//! ```no_run
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
//! let response = client.send(request).expect("Failed to send request");
//! let data = response.into_body_str().expect("Failed to parse body");
//! ```
//!
//! # Custom Base URL
//!
//! You can configure a custom base URL for the Gate.io API:
//!
//! ```
//! use gateio_rs::ureq::GateHttpClient;
//!
//! let client = GateHttpClient::with_url("https://api.gateio.ws");
//! ```
//!
//! # Error Handling
//!
//! All errors emitted by the client implement the standard [`Error`] trait.
//!
//! # Custom Timeout
//!
//! Configure custom timeouts using ureq's config builder:
//!
//! ```
//! use ureq::Agent;
//! use gateio_rs::ureq::GateHttpClient;
//! use std::time::Duration;
//!
//! let agent: Agent = Agent::config_builder()
//!     .timeout_request(Duration::from_secs(10))
//!     .timeout_connect(Duration::from_secs(5))
//!     .build()
//!     .into();
//!
//! let client = GateHttpClient::with_custom_agent(agent, "https://api.gateio.ws");
//! ```

mod client;
mod error;
mod response;

pub use client::*;
pub use error::*;
pub use response::*;
