use serde_json::{Map, Value};
use crate::http::{request::Request, Credentials, Method};

pub struct GetAccount {
    pub currency: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetAccount {
    pub fn new() -> Self {
        Self {
            currency: None,
            credentials: None,
        }
    }

    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.into());
        self
    }

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

        let payload = Map::new();

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Get,
            path: "/api/v4/spot/accounts".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}