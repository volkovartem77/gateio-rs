use crate::http::{request::Request, Credentials, Method};

pub struct GetAccount {
    pub currency: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetAccount {
    pub fn new() -> Self { Self { currency: None, credentials: None } }

    pub fn currency(mut self, s: &str) -> Self { self.currency = Some(s.into()); self }

    pub fn credentials(mut self, creds: Credentials) -> Self { self.credentials = Some(creds); self }
}

impl From<GetAccount> for Request {
    fn from(g: GetAccount) -> Request {
        let mut params = Vec::new();
        if let Some(s) = g.currency { params.push(("currency".into(), s)); }

        let mut payload = Vec::new();

        Request {
            method: Method::Get,
            path: "/api/v4/spot/accounts".into(),
            params,
            payload,
            x_gate_exp_time: None,
            credentials: g.credentials,
            sign: true,
        }
    }
}