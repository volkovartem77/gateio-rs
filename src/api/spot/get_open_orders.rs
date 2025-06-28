use crate::http::{Credentials, Method, request::Request};

pub struct GetOpenOrders {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub account: Option<String>,
    pub credentials: Option<Credentials>,
}

impl GetOpenOrders {
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
            account: None,
            credentials: None,
        }
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetOpenOrders> for Request {
    fn from(request: GetOpenOrders) -> Request {
        let mut params = Vec::new();

        if let Some(page) = request.page {
            params.push(("page".into(), page.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(account) = request.account {
            params.push(("account".into(), account));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/open_orders".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
