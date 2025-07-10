use crate::http::{Credentials, Method, request::Request};

/// # Cancel all price-triggered orders
///
/// Cancel all running price-triggered orders (auto orders/conditional orders).
/// This operation will cancel all currently active price-triggered orders for the account.
///
/// ## Important Notes:
/// - This action cannot be undone
/// - Only cancels orders in "open" status (waiting to trigger)
/// - Orders that have already triggered and are executing cannot be cancelled
/// - Completed, failed, expired orders are not affected
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-all-price-triggered-orders)
pub struct CancelAllPriceOrders {
    pub market: Option<String>,
    pub account: Option<String>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CancelAllPriceOrders {
    pub fn new() -> Self {
        Self {
            market: None,
            account: None,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    /// Filter by currency pair (market) - if specified, only orders for this market will be cancelled
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

    /// Specify the expiration time (milliseconds);
    /// If the GATE receives the request time greater than the expiration time, the request will be rejected
    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time);
        self
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CancelAllPriceOrders> for Request {
    fn from(request: CancelAllPriceOrders) -> Request {
        let mut params = Vec::new();

        if let Some(market) = request.market {
            params.push(("market".into(), market));
        }

        if let Some(account) = request.account {
            params.push(("account".into(), account));
        }

        Request {
            method: Method::Delete,
            path: "/api/v4/spot/price_orders".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
