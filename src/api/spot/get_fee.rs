use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving trading fees for spot trading
pub struct GetFee {
    /// Optional currency pair to get specific trading fees for
    pub currency_pair: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetFee {
    /// Creates a new GetFee request
    pub fn new() -> Self {
        Self {
            currency_pair: None,
            credentials: None,
        }
    }

    /// Sets the currency pair to get specific fees for
    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.into());
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetFee> for Request {
    fn from(request: GetFee) -> Request {
        let mut params = Vec::new();
        if let Some(currency_pair) = request.currency_pair {
            params.push(("currency_pair".into(), currency_pair));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/fee".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
