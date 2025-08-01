use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving details of a specific order
pub struct GetOrder {
    /// Order ID to retrieve details for
    pub order_id: String,
    /// Currency pair the order belongs to
    pub currency_pair: String,
    /// Optional account type filter
    pub account: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl GetOrder {
    /// Creates a new GetOrder request with order ID and currency pair
    pub fn new(order_id: &str, currency_pair: &str) -> Self {
        Self {
            order_id: order_id.into(),
            currency_pair: currency_pair.into(),
            account: None,
            credentials: None,
        }
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

impl From<GetOrder> for Request {
    fn from(request: GetOrder) -> Request {
        let mut params = Vec::new();

        params.push(("currency_pair".into(), request.currency_pair));

        if let Some(account) = request.account {
            params.push(("account".into(), account));
        }

        Request {
            method: Method::Get,
            path: format!("/api/v4/spot/orders/{}", request.order_id),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
