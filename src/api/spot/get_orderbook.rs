use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving order book data for a currency pair
pub struct GetOrderbook {
    /// Currency pair to get order book for
    pub currency_pair: String,
    /// Price interval aggregation ("0" for no aggregation)
    pub interval: Option<String>,
    /// Maximum depth of order book entries to return
    pub limit: Option<i64>,
    /// Whether to return order IDs with the book data
    pub with_id: Option<bool>,
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetOrderbook {
    /// Creates a new GetOrderbook request for the specified currency pair
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            interval: None,
            limit: None,
            with_id: None,
            credentials: None,
        }
    }

    /// Sets the price interval for aggregation
    pub fn interval(mut self, interval: &str) -> Self {
        self.interval = Some(interval.into());
        self
    }

    /// Sets the maximum depth of order book entries
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit.into());
        self
    }

    /// Sets whether to include order IDs in the response
    pub fn with_id(mut self, with_id: bool) -> Self {
        self.with_id = Some(with_id);
        self
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetOrderbook> for Request {
    fn from(request: GetOrderbook) -> Request {
        let mut params = vec![("currency_pair".to_owned(), request.currency_pair)];

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
