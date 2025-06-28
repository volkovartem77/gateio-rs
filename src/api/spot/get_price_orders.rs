use crate::http::{request::Request, Credentials, Method};

pub struct GetPriceOrders {
    pub status: Option<String>,
    pub currency_pair: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub credentials: Option<Credentials>,
}

impl GetPriceOrders {
    pub fn new() -> Self {
        Self {
            status: None,
            currency_pair: None,
            from: None,
            to: None,
            page: None,
            limit: None,
            credentials: None,
        }
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.into());
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

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetPriceOrders> for Request {
    fn from(request: GetPriceOrders) -> Request {
        let mut params = Vec::new();
        
        if let Some(status) = request.status {
            params.push(("status".into(), status));
        }

        if let Some(currency_pair) = request.currency_pair {
            params.push(("currency_pair".into(), currency_pair));
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

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/price_orders".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}