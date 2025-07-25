use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving spot account information
pub struct GetAccount {
    /// Optional currency filter to get balances for a specific currency
    pub currency: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetAccount {
    /// Creates a new GetAccount request
    pub fn new() -> Self {
        Self {
            currency: None,
            credentials: None,
        }
    }

    /// Sets the currency filter for the account query
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetAccount> for Request {
    fn from(request: GetAccount) -> Request {
        let mut params = Vec::new();
        if let Some(currency) = request.currency {
            params.push(("currency".into(), currency));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/accounts".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
