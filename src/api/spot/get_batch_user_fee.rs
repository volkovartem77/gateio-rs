use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving trading fees for multiple currency pairs
pub struct GetBatchUserFee {
    /// Comma-separated list of currency pairs to get fees for
    pub currency_pairs: String,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetBatchUserFee {
    /// Creates a new GetBatchUserFee request with currency pairs
    pub fn new(currency_pairs: &str) -> Self {
        Self {
            currency_pairs: currency_pairs.to_owned(),
            credentials: None,
        }
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetBatchUserFee> for Request {
    fn from(request: GetBatchUserFee) -> Request {
        let params = vec![("currency_pairs".to_owned(), request.currency_pairs)];

        Request {
            method: Method::Get,
            path: "/api/v4/spot/batch_fee".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
