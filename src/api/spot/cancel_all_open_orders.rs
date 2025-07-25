use crate::http::{Credentials, Method, request::Request};

/// Cancel all open orders request
pub struct CancelAllOpenOrders {
    /// Trading pair filter
    pub currency_pair: Option<String>,
    /// API credentials
    pub credentials: Option<Credentials>,
}

impl CancelAllOpenOrders {
    /// Create cancel all request
    pub fn new() -> Self {
        Self {
            currency_pair: None,
            credentials: None,
        }
    }

    /// Filter by trading pair
    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.to_string());
        self
    }

    /// Set API credentials
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CancelAllOpenOrders> for Request {
    fn from(request: CancelAllOpenOrders) -> Request {
        let mut params = Vec::new();

        if let Some(currency_pair) = &request.currency_pair {
            params.push(("currency_pair".to_string(), currency_pair.clone()));
        }

        Request {
            method: Method::Delete,
            path: "/api/v4/spot/orders".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
