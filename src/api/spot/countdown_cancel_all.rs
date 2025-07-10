use crate::http::{Credentials, Method, request::Request};
use serde_json::{Map, Value, json};

pub struct CountdownCancelAll {
    pub timeout: i64,
    pub currency_pair: Option<String>,
    pub credentials: Option<Credentials>,
}

impl CountdownCancelAll {
    pub fn new(timeout: i64) -> Self {
        Self {
            timeout,
            currency_pair: None,
            credentials: None,
        }
    }

    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.to_string());
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CountdownCancelAll> for Request {
    fn from(request: CountdownCancelAll) -> Request {
        let params = Vec::new();
        let mut payload = Map::new();

        payload.insert("timeout".to_string(), json!(request.timeout));

        if let Some(currency_pair) = request.currency_pair {
            payload.insert("currency_pair".to_string(), json!(currency_pair));
        }

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Post,
            path: "/api/v4/spot/countdown_cancel_all".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
