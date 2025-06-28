use crate::http::{request::Request, Credentials, Method};

pub struct GetCandlesticks {
    pub currency_pair: String,
    pub limit: Option<i64>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub interval: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetCandlesticks {
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            limit: None,
            from: None,
            to: None,
            interval: None,
            credentials: None,
        }
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit.into());
        self
    }

    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn interval(mut self, interval: &str) -> Self {
        self.interval = Some(interval.into());
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCandlesticks> for Request {
    fn from(request: GetCandlesticks) -> Request {
        let mut params = vec![
            ("currency_pair".to_owned(), request.currency_pair),
        ];

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(from) = request.from {
            params.push(("from".into(), from.to_string()));
        }

        if let Some(to) = request.to {
            params.push(("to".into(), to.to_string()));
        }

        if let Some(interval) = request.interval {
            params.push(("interval".into(), interval.to_string()));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/candlesticks".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}