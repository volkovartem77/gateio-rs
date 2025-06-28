use crate::http::{Credentials, Method, request::Request};

pub struct GetMyTrades {
    pub currency_pair: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub order_id: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetMyTrades {
    pub fn new() -> Self {
        Self {
            currency_pair: None,
            page: None,
            limit: None,
            from: None,
            to: None,
            order_id: None,
            credentials: None,
        }
    }

    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.into());
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }

    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to);
        self
    }

    pub fn order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetMyTrades> for Request {
    fn from(request: GetMyTrades) -> Request {
        let mut params = Vec::new();

        if let Some(currency_pair) = request.currency_pair {
            params.push(("currency_pair".into(), currency_pair));
        }

        if let Some(page) = request.page {
            params.push(("page".into(), page.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(from) = request.from {
            params.push(("from".into(), from.to_string()));
        }

        if let Some(to) = request.to {
            params.push(("to".into(), to.to_string()));
        }

        if let Some(order_id) = request.order_id {
            params.push(("order_id".into(), order_id));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/my_trades".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}