use crate::http::{Credentials, Method, request::Request};

/// Request builder for retrieving ticker information.
///
/// Gets 24hr trading statistics for currency pairs including price, volume,
/// and percentage changes. Can be used for a specific pair or all pairs.
///
/// # API Endpoint
/// `GET /api/v4/spot/tickers`
///
/// # Examples
///
/// ```rust,no_run
/// use gateio_rs::{api::spot::get_ticker, ureq::GateHttpClient};
///
/// let client = GateHttpClient::default();
///
/// // Get ticker for specific pair
/// let request = get_ticker()
///     .currency_pair("BTC_USDT")
///     .timezone("utc8");
/// let response = client.send(request)?;
///
/// // Get all tickers
/// let request = get_ticker();
/// let response = client.send(request)?;
/// # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
/// ```
///
/// # Response Format
/// Returns ticker data with fields like:
/// - `currency_pair`: Trading pair name
/// - `last`: Last traded price
/// - `lowest_ask`: Lowest ask price
/// - `highest_bid`: Highest bid price
/// - `change_percentage`: 24hr price change percentage
/// - `base_volume`: 24hr base currency volume
/// - `quote_volume`: 24hr quote currency volume
/// - `high_24h`: 24hr highest price
/// - `low_24h`: 24hr lowest price
pub struct GetTicker {
    /// Optional currency pair to get ticker for (if not specified, returns all tickers)
    pub currency_pair: Option<String>,
    /// Timezone for the ticker data timestamps
    pub timezone: Option<String>,
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetTicker {
    /// Creates a new GetTicker request
    pub fn new() -> Self {
        Self {
            currency_pair: None,
            timezone: None,
            credentials: None,
        }
    }

    /// Sets the currency pair to get ticker for
    pub fn currency_pair(mut self, s: &str) -> Self {
        self.currency_pair = Some(s.into());
        self
    }

    /// Sets the timezone for timestamp data
    pub fn timezone(mut self, tz: &str) -> Self {
        self.timezone = Some(tz.into());
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetTicker> for Request {
    fn from(g: GetTicker) -> Request {
        let mut params = Vec::new();
        if let Some(s) = g.currency_pair {
            params.push(("currency_pair".into(), s));
        }
        if let Some(tz) = g.timezone {
            params.push(("timezone".into(), tz));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/tickers".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: g.credentials,
            sign: false,
        }
    }
}
