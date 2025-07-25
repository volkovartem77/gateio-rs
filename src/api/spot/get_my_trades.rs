use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving user's personal trading history
pub struct GetMyTrades {
    /// Optional currency pair filter for trades
    pub currency_pair: Option<String>,
    /// Page number for pagination
    pub page: Option<i32>,
    /// Maximum number of trades to return per page
    pub limit: Option<i32>,
    /// Start timestamp for trade history range
    pub from: Option<i64>,
    /// End timestamp for trade history range
    pub to: Option<i64>,
    /// Optional order ID to filter trades by
    pub order_id: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetMyTrades {
    /// Creates a new GetMyTrades request
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

    /// Sets the currency pair filter for trades
    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.into());
        self
    }

    /// Sets the page number for pagination
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the maximum number of trades per page
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the start timestamp for trade history range
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }

    /// Sets the end timestamp for trade history range
    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to);
        self
    }

    /// Sets the order ID filter for trades
    pub fn order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    /// Sets the API credentials for authentication
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
