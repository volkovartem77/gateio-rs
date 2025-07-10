use crate::http::{Credentials, Method, request::Request};

pub struct GetFee {
    pub currency_pair: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetFee {
    pub fn new() -> Self {
        Self {
            currency_pair: None,
            credentials: None,
        }
    }

    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.into());
        self
    }

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
