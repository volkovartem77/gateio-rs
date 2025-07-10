use crate::http::{Credentials, Method, request::Request};

/// # Retrieve running auto order list
///
/// Retrieve running price-triggered orders (also called auto orders or conditional orders).
/// These orders are not placed in the order book until their trigger condition is met.
///
/// ## Status Values:
/// - "open": Waiting to trigger
/// - "cancelled": Manually cancelled  
/// - "finish": Successfully executed
/// - "failed": Failed to execute
/// - "expired": Expired
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#retrieve-running-auto-order-list)
pub struct GetPriceOrders {
    pub status: Option<String>,
    pub market: Option<String>,
    pub account: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub credentials: Option<Credentials>,
}

impl GetPriceOrders {
    pub fn new() -> Self {
        Self {
            status: None,
            market: None,
            account: None,
            limit: None,
            offset: None,
            credentials: None,
        }
    }

    /// Filter by order status
    /// - "open": Waiting to trigger
    /// - "cancelled": Manually cancelled  
    /// - "finish": Successfully executed
    /// - "failed": Failed to execute
    /// - "expired": Expired
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by currency pair (market)
    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.into());
        self
    }

    /// Filter by account type
    /// - "normal": Normal spot trading account
    /// - "margin": Margin trading account  
    /// - "unified": Unified trading account
    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    /// Maximum number of records to return (default: 100, max: 1000)
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Offset for pagination (default: 0)
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
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

        if let Some(market) = request.market {
            params.push(("market".into(), market));
        }

        if let Some(account) = request.account {
            params.push(("account".into(), account));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".into(), limit.to_string()));
        }

        if let Some(offset) = request.offset {
            params.push(("offset".into(), offset.to_string()));
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
