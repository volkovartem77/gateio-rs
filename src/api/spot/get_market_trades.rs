use crate::http::{Credentials, Method, request::Request};

pub struct GetMarketTrades {
    pub currency_pair: String,
    pub limit: Option<i32>,
    pub last_id: Option<String>,
    pub reverse: Option<bool>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub page: Option<i32>,
    pub credentials: Option<Credentials>,
}

impl GetMarketTrades {
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            limit: None,
            last_id: None,
            reverse: None,
            from: None,
            to: None,
            page: None,
            credentials: None,
        }
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit.into());
        self
    }

    pub fn last_id(mut self, last_id: &str) -> Self {
        self.last_id = Some(last_id.into());
        self
    }

    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = Some(reverse.into());
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

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page.into());
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetMarketTrades> for Request {
    fn from(request: GetMarketTrades) -> Request {
        let mut params = vec![("currency_pair".to_owned(), request.currency_pair)];

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(last_id) = request.last_id {
            params.push(("last_id".into(), last_id.to_string()));
        }

        if let Some(reverse) = request.reverse {
            params.push(("reverse".into(), reverse.to_string()));
        }

        if let Some(from) = request.from {
            params.push(("from".into(), from.to_string()));
        }

        if let Some(to) = request.to {
            params.push(("to".into(), to.to_string()));
        }

        if let Some(page) = request.page {
            params.push(("page".into(), page.to_string()));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/trades".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}
