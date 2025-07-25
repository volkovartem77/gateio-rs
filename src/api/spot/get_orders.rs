use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving order history with various filtering options
pub struct GetOrders {
    /// Optional currency pair filter for orders
    pub currency_pair: Option<String>,
    /// Order status filter ("open", "closed", "cancelled")
    pub status: Option<String>,
    /// Page number for pagination
    pub page: Option<i32>,
    /// Maximum number of orders to return per page
    pub limit: Option<i32>,
    /// Optional account type filter
    pub account: Option<String>,
    /// Start timestamp for order history range
    pub from: Option<i64>,
    /// End timestamp for order history range
    pub to: Option<i64>,
    /// Order side filter ("buy" or "sell")
    pub side: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetOrders {
    /// Creates a new GetOrders request
    pub fn new() -> Self {
        Self {
            currency_pair: None,
            status: None,
            page: None,
            limit: None,
            account: None,
            from: None,
            to: None,
            side: None,
            credentials: None,
        }
    }

    /// Sets the currency pair filter for orders
    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.into());
        self
    }

    /// Sets the order status filter
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.into());
        self
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

    /// Sets the start timestamp for order history range
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }

    /// Sets the end timestamp for order history range
    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to);
        self
    }

    /// Sets the order side filter ("buy" or "sell")
    pub fn side(mut self, side: &str) -> Self {
        self.side = Some(side.into());
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetOrders> for Request {
    fn from(request: GetOrders) -> Request {
        let mut params = Vec::new();

        if let Some(currency_pair) = request.currency_pair {
            params.push(("currency_pair".into(), currency_pair));
        }

        if let Some(status) = request.status {
            params.push(("status".into(), status));
        }

        if let Some(page) = request.page {
            params.push(("page".into(), page.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(account) = request.account {
            params.push(("account".into(), account));
        }

        if let Some(from) = request.from {
            params.push(("from".into(), from.to_string()));
        }

        if let Some(to) = request.to {
            params.push(("to".into(), to.to_string()));
        }

        if let Some(side) = request.side {
            params.push(("side".into(), side));
        }

        Request {
            method: Method::Get,
            path: "/api/v4/spot/orders".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
