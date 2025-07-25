use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving all open orders
pub struct GetOpenOrders {
    /// Page number for pagination
    pub page: Option<i32>,
    /// Maximum number of orders to return per page
    pub limit: Option<i32>,
    /// Optional account type filter
    pub account: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetOpenOrders {
    /// Creates a new GetOpenOrders request
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
            account: None,
            credentials: None,
        }
    }

    /// Sets the page number for pagination
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the maximum number of orders per page
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the account type filter
    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    /// Sets the API credentials for authentication
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
