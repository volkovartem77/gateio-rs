use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving information about a specific currency pair
pub struct GetCurrencyPair {
    /// Currency pair to get information for (e.g., "BTC_USDT")
    pub currency_pair: String,
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetCurrencyPair {
    /// Creates a new GetCurrencyPair request for the specified pair
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            credentials: None,
        }
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCurrencyPair> for Request {
    fn from(request: GetCurrencyPair) -> Request {
        let params = Vec::new();

        Request {
            method: Method::Get,
            path: format!("/api/v4/spot/currency_pairs/{}", request.currency_pair).into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}
