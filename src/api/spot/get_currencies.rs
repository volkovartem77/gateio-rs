use crate::http::{request::Request, Credentials, Method};

pub struct GetCurrencies {
    pub credentials: Option<Credentials>,
}

impl GetCurrencies {
    pub fn new() -> Self { Self { credentials: None } }

    pub fn credentials(mut self, creds: Credentials) -> Self { self.credentials = Some(creds); self }
}

impl From<GetCurrencies> for Request {
    fn from(request: GetCurrencies) -> Request {
        let mut params = Vec::new();
        let mut payload = Vec::new();

        Request {
            method: Method::Get,
            path: "/api/v4/spot/currencies".into(),
            params,
            payload,
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}