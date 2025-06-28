use crate::http::{request::Request, Credentials, Method};

pub struct GetOrderbook {
    pub currency_pair: String,
    pub interval: Option<String>,
    pub limit: Option<i64>,
    pub with_id: Option<bool>,
    pub credentials: Option<Credentials>,
}

impl GetOrderbook {
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            interval: None,
            limit: None,
            with_id: None,
            credentials: None,
        }
    }

    pub fn interval(mut self, interval: &str) -> Self {
        self.interval = Some(interval.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit.into());
        self
    }

    pub fn with_id(mut self, limit: bool) -> Self {
        self.limit = Some(limit.into());
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetOrderbook> for Request {
    fn from(request: GetOrderbook) -> Request {
        let mut params = vec![
            ("currency_pair".to_owned(), request.currency_pair),
        ];

        if let Some(interval) = request.interval {
            params.push(("interval".into(), interval.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(with_id) = request.with_id {
            params.push(("with_id".into(), with_id.to_string()));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/order_book".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}