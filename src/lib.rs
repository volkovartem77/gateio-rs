//! Gate.io client library (каркас).
//!
//! ```rust,no_run
//! use gateio_rs::Client;
//!
//! // асинхронный пример (фича "async" по умолчанию)
//! # #[tokio::main]
//! # async fn main() -> gateio_rs::Result<()> {
//! let cli = Client::new("API_KEY", "API_SECRET");
//! let price = cli.ticker("BTC_USDT").await?;
//! println!("btc/usdt = {price}");
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

mod error;
pub use error::{Error, Result};

pub mod client;
pub use client::Client;

//
// —--- Публичные трейты  ---—
//

/// Асинхронный интерфейс к Gate.io.
///
/// Доступен, когда активен feature **`async`** (включён по умолчанию).
#[cfg(feature = "rt-tokio")]
#[async_trait::async_trait]
pub trait GateClientAsync {
    /// Получить текущий тикер для торговой пары, например `"BTC_USDT"`.
    async fn ticker(&self, symbol: &str) -> Result<f64>;
}

/// Блокирующий (sync) интерфейс.
///
/// Подключается фичей **`blocking`**.
#[cfg(feature = "blocking")]
pub trait GateClientSync {
    /// Синхронная версия `ticker`.
    fn ticker(&self, symbol: &str) -> Result<f64>;
}