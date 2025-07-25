use super::order::Order;
use crate::http::{Credentials, Method, request::Request};

/// # Create multiple spot orders in batch
///
/// Create multiple spot orders in a single request for improved efficiency.
/// All orders will be processed together and either all succeed or all fail.
///
/// ## Important Notes:
/// - Maximum 10 orders per batch request
/// - All orders must be for the same account type
/// - Orders are processed atomically (all or nothing)
/// - Each order follows the same validation rules as individual orders
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#create-a-batch-of-orders)
pub struct CreateBatchOrders {
    /// List of orders to create (maximum 10)
    pub orders: Vec<Order>,
    /// Request expiration time in milliseconds
    pub x_gate_exp_time: Option<u128>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl CreateBatchOrders {
    /// Create a new batch orders request
    pub fn new(orders: Vec<Order>) -> Self {
        Self {
            orders,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    /// Set the request expiration time in milliseconds
    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time);
        self
    }

    /// Set API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CreateBatchOrders> for Request {
    fn from(request: CreateBatchOrders) -> Request {
        let params = Vec::new();
        let payload = serde_json::to_string(&request.orders).unwrap();

        Request {
            method: Method::Post,
            path: "/api/v4/spot/batch_orders".into(),
            params,
            payload,
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
