use serde_json::{Map, Value};
use crate::http::{request::Request, Credentials, Method};

pub struct GetTicker {
    pub currency_pair: Option<String>,
    pub timezone: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetTicker {
    pub fn new() -> Self { Self { currency_pair: None, timezone: None, credentials: None } }

    pub fn currency_pair(mut self, s: &str) -> Self { self.currency_pair = Some(s.into()); self }

    pub fn timezone(mut self, tz: &str) -> Self { self.timezone = Some(tz.into()); self }

    pub fn credentials(mut self, creds: Credentials) -> Self { self.credentials = Some(creds); self }
}

impl From<GetTicker> for Request {
    fn from(g: GetTicker) -> Request {
        let mut params = Vec::new();
        let payload = Map::new();
        if let Some(s) = g.currency_pair { params.push(("currency_pair".into(), s)); }
        if let Some(tz) = g.timezone { params.push(("timezone".into(), tz)); }

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Get,
            path: "/api/v4/spot/tickers".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: None,
            credentials: g.credentials,
            sign: false,
        }
    }
}