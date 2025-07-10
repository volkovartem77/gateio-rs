use crate::http::{Credentials, Method, request::Request};

/// # Cancel a price-triggered order
///
/// Cancel a specific price-triggered order (auto order/conditional order) by order ID.
/// Only orders in "open" status (waiting to trigger) can be cancelled.
///
/// ## Important Notes:
/// - Only orders in "open" status can be cancelled
/// - Orders that have already triggered and are executing cannot be cancelled
/// - Completed, failed, expired orders cannot be cancelled
/// - This action cannot be undone
///
/// ## Response:
/// Returns the cancelled order details with updated status
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-a-price-triggered-order)
pub struct CancelPriceOrder {
    pub order_id: String,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CancelPriceOrder {
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.to_owned(),
            x_gate_exp_time: None,
            credentials: None,
        }
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

impl From<CancelPriceOrder> for Request {
    fn from(request: CancelPriceOrder) -> Request {
        let params = Vec::new();

        Request {
            method: Method::Delete,
            path: format!("/api/v4/spot/price_orders/{}", request.order_id).into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
