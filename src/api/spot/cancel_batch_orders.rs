use crate::http::{Credentials, Method, request::Request};
use serde::Serialize;

/// Single order cancellation parameters
#[derive(Serialize)]
pub struct CancelOrderRequest {
    /// Order ID to cancel
    pub id: String,
    /// Trading pair
    pub currency_pair: String,
}

/// Batch order cancellation request
pub struct CancelBatchOrders {
    /// Orders to cancel
    pub orders: Vec<CancelOrderRequest>,
    /// Request expiration time
    pub x_gate_exp_time: Option<u128>,
    /// API credentials
    pub credentials: Option<Credentials>,
}

impl CancelBatchOrders {
    /// Create batch cancellation request
    pub fn new(orders: Vec<CancelOrderRequest>) -> Self {
        Self {
            orders,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    /// Set expiration time
    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time);
        self
    }

    /// Set API credentials
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CancelBatchOrders> for Request {
    fn from(request: CancelBatchOrders) -> Request {
        let params = Vec::new();
        let payload = serde_json::to_string(&request.orders).unwrap();

        Request {
            method: Method::Post,
            path: "/api/v4/spot/cancel_batch_orders".into(),
            params,
            payload,
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
