use serde_json::{Map, Value};
use crate::http::{Credentials, Method, request::Request};

pub struct GetCurrency {
    pub currency: String,
    pub credentials: Option<Credentials>,
}

impl GetCurrency {
    pub fn new(currency: &str) -> Self {
        Self {
            currency: currency.to_owned(),
            credentials: None
        }
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCurrency> for Request {
    fn from(request: GetCurrency) -> Request {
        let mut params = Vec::new();
        let payload = Map::new();

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Get,
            path: format!("/api/v4/spot/currencies/{}", request.currency).into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}