use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving candlestick/kline data for a currency pair
pub struct GetCandlesticks {
    /// Currency pair to get candlestick data for
    pub currency_pair: String,
    /// Maximum number of candlesticks to return
    pub limit: Option<i64>,
    /// Start timestamp for candlestick data range
    pub from: Option<i64>,
    /// End timestamp for candlestick data range
    pub to: Option<i64>,
    /// Time interval for candlesticks (e.g., "1m", "5m", "1h")
    pub interval: Option<String>,
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetCandlesticks {
    /// Creates a new GetCandlesticks request for the specified currency pair
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            limit: None,
            from: None,
            to: None,
            interval: None,
            credentials: None,
        }
    }

    /// Sets the maximum number of candlesticks to return
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit.into());
        self
    }

    /// Sets the start timestamp for the data range
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the end timestamp for the data range
    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Sets the time interval for candlesticks
    pub fn interval(mut self, interval: &str) -> Self {
        self.interval = Some(interval.into());
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCandlesticks> for Request {
    fn from(request: GetCandlesticks) -> Request {
        let mut params = vec![("currency_pair".to_owned(), request.currency_pair)];

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(from) = request.from {
            params.push(("from".into(), from.to_string()));
        }

        if let Some(to) = request.to {
            params.push(("to".into(), to.to_string()));
        }

        if let Some(interval) = request.interval {
            params.push(("interval".into(), interval.to_string()));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/candlesticks".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}
