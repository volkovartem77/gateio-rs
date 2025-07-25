use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving the server's current time
pub struct GetServerTime {
    /// API credentials for authentication (optional for public endpoint)
    pub credentials: Option<Credentials>,
}

impl GetServerTime {
    /// Creates a new GetServerTime request
    pub fn new() -> Self {
        Self { credentials: None }
    }

    /// Sets the API credentials for authentication
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
