use serde_json::{Map, Value};
use crate::http::{request::Request, Credentials, Method};

pub struct GetCurrencyPairs {
    pub credentials: Option<Credentials>,
}

impl GetCurrencyPairs {
    pub fn new() -> Self { Self { credentials: None } }

    pub fn credentials(mut self, creds: Credentials) -> Self { self.credentials = Some(creds); self }
}

impl From<GetCurrencyPairs> for Request {
    fn from(request: GetCurrencyPairs) -> Request {
        let mut params = Vec::new();
        let payload = Map::new();

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Get,
            path: "/api/v4/spot/currency_pairs".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}