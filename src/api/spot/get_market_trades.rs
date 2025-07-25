use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving recent public trades for a currency pair
pub struct GetMarketTrades {
    /// Currency pair to get trades for
    pub currency_pair: String,
    /// Maximum number of trades to return
    pub limit: Option<i32>,
    /// Last trade ID for pagination
    pub last_id: Option<String>,
    /// Whether to reverse the order of results
    pub reverse: Option<bool>,
    /// Start timestamp for trade history range
    pub from: Option<i64>,
    /// End timestamp for trade history range
    pub to: Option<i64>,
    /// Page number for pagination
    pub page: Option<i32>,
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetMarketTrades {
    /// Creates a new GetMarketTrades request for the specified currency pair
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

    /// Sets the maximum number of trades to return
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit.into());
        self
    }

    /// Sets the last trade ID for pagination
    pub fn last_id(mut self, last_id: &str) -> Self {
        self.last_id = Some(last_id.into());
        self
    }

    /// Sets whether to reverse the order of results
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = Some(reverse.into());
        self
    }

    /// Sets the start timestamp for trade history range
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the end timestamp for trade history range
    pub fn to(mut self, to: i64) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Sets the page number for pagination
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sets the API credentials for authentication
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
