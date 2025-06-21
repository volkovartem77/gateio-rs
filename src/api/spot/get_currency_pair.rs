use crate::http::{Credentials, Method, request::Request};

pub struct GetCurrencyPair {
    pub currency_pair: String,
    pub credentials: Option<Credentials>,
}

impl GetCurrencyPair {
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            credentials: None
        }
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCurrencyPair> for Request {
    fn from(request: GetCurrencyPair) -> Request {
        let mut params = Vec::new();
        let mut payload = Vec::new();

        Request {
            method: Method::Get,
            path: format!("/api/v4/spot/currency_pairs/{}", request.currency_pair).into(),
            params,
            payload,
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}