use crate::http::{Credentials, Method, request::Request};

pub struct GetBatchUserFee {
    pub currency_pairs: String,
    pub credentials: Option<Credentials>,
}

impl GetBatchUserFee {
    pub fn new(currency_pairs: &str) -> Self {
        Self {
            currency_pairs: currency_pairs.to_owned(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetBatchUserFee> for Request {
    fn from(request: GetBatchUserFee) -> Request {
        let mut params = vec![("currency_pairs".to_owned(), request.currency_pairs)];

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
