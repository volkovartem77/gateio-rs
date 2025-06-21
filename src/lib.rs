//! Gate.io Rust SDK — асинхронный и синхронный HTTP-клиент
//!
//! ```rust,no_run
//! ```
//!
//! The following optional features are available:
//!
//! * `enable-ureq`: For a blocking http client powered by [`ureq`](https://docs.rs/ureq/2.4.0/ureq/).
//! * `enable-hyper`: For a non-blocking http client powered by [`hyper`](https://docs.rs/hyper/0.14.16/hyper/).
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

