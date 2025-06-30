use crate::http::{request::Request, Credentials, Method};

pub struct GetServerTime {
    pub credentials: Option<Credentials>,
}

impl GetServerTime {
    pub fn new() -> Self {
        Self {
            credentials: None,
        }
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetServerTime> for Request {
    fn from(request: GetServerTime) -> Request {
        let params = Vec::new();

        Request {
            method: Method::Get,
            path: "/api/v4/spot/time".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}