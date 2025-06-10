//! Реализации клиента Gate.io под разные рантаймы.

use crate::{Error, Result};

/// Ключи доступа к API.
#[derive(Clone)]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
}

impl Credentials {
    pub fn new<K: Into<String>, S: Into<String>>(key: K, secret: S) -> Self {
        Self {
            api_key: key.into(),
            api_secret: secret.into(),
        }
    }
}

/// Единый «конструктор», который внутри выбирает конкретную реализацию.
///
/// * При активном **`rt-tokio`** возвращает `AsyncClient`.
/// * При активном **только** `blocking` — `BlockingClient`.
pub struct Client {
    inner: Inner,
}

enum Inner {
    #[cfg(feature = "rt-tokio")]
    Async(AsyncClient),

    #[cfg(feature = "blocking")]
    Blocking(BlockingClient),
}

impl Client {
    /// Создать новый клиент.
    pub fn new<K: Into<String>, S: Into<String>>(key: K, secret: S) -> Self {
        let creds = Credentials::new(key, secret);

        #[cfg(feature = "rt-tokio")]
        {
            Self {
                inner: Inner::Async(AsyncClient::new(creds)),
            }
        }

        #[cfg(all(not(feature = "rt-tokio"), feature = "blocking"))]
        {
            Self {
                inner: Inner::Blocking(BlockingClient::new(creds)),
            }
        }
    }

    // ---------- публичный «универсальный» API -----------
    #[cfg(feature = "rt-tokio")]
    /// Асинхронный тикер (доступен при включённой фиче `rt-tokio`).
    pub async fn ticker(&self, symbol: &str) -> Result<f64> {
        let Inner::Async(cli) = &self.inner;
        cli.ticker(symbol).await
    }

    #[cfg(feature = "blocking")]
    /// Блокирующий тикер (доступен при фиче `blocking` **и** выключенной `rt-tokio`).
    pub fn ticker_blocking(&self, symbol: &str) -> Result<f64> {
        match &self.inner {
            Inner::Blocking(cli) => cli.ticker(symbol),
            _ => unreachable!("blocking feature expected"),
        }
    }
}

//
// —--- ASYNC реализация  ---—
//
#[cfg(feature = "rt-tokio")]
mod async_impl {
    use super::*;
    use reqwest::Client as Http;

    pub struct AsyncClient {
        creds: Credentials,
        http: Http,
    }

    impl AsyncClient {
        pub fn new(creds: Credentials) -> Self {
            Self {
                creds,
                http: Http::new(),
            }
        }

        pub async fn ticker(&self, _symbol: &str) -> Result<f64> {
            // TODO: сформировать запрос, подписать, отправить...
            // todo!("call async /spot/tickers endpoint")
            println!("[async] fetching ticker for {}", _symbol);
            Ok(654.321)
        }
    }

    #[async_trait::async_trait]
    impl crate::GateClientAsync for AsyncClient {
        async fn ticker(&self, symbol: &str) -> Result<f64> {
            self.ticker(symbol).await
        }
    }
}
#[cfg(feature = "rt-tokio")]
pub use async_impl::AsyncClient;

//
// —--- BLOCKING реализация  ---—
//
#[cfg(feature = "blocking")]
mod blocking_impl {
    use super::*;
    use reqwest::blocking::Client as Http;

    pub struct BlockingClient {
        creds: Credentials,
        http: Http,
    }

    impl BlockingClient {
        pub fn new(creds: Credentials) -> Self {
            Self {
                creds,
                http: Http::new(),
            }
        }

        pub fn ticker(&self, _symbol: &str) -> Result<f64> {
            // TODO: тот же эндпоинт, но в блокирующем варианте
            // todo!("call blocking /spot/tickers endpoint")
            println!("[sync] fetching ticker for {}", _symbol);
            // вернём, например, фиктивное значение
            Ok(123.456)
        }
    }

    impl crate::GateClientSync for BlockingClient {
        fn ticker(&self, symbol: &str) -> Result<f64> {
            self.ticker(symbol)
        }
    }
}
#[cfg(feature = "blocking")]
pub use blocking_impl::BlockingClient;