use crate::http::{Credentials, Method, request::Request};

/// # Get a price-triggered order
///
/// Retrieve details of a specific price-triggered order (auto order/conditional order) by order ID.
///
/// ## Order Status Values:
/// - "open": Waiting to trigger
/// - "cancelled": Manually cancelled  
/// - "finish": Successfully executed
/// - "failed": Failed to execute
/// - "expired": Expired
///
/// ## Response includes:
/// - Order ID and creation time
/// - Trigger condition (price and rule)
/// - Order details (currency pair, side, amount, price)
/// - Current status and execution details
/// - Account information
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#get-a-price-triggered-order)
pub struct GetPriceOrder {
    pub order_id: String,
    pub credentials: Option<Credentials>,
}

impl GetPriceOrder {
    pub fn new(order_id: &str) -> Self {
        Self {
            order_id: order_id.to_owned(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetPriceOrder> for Request {
    fn from(request: GetPriceOrder) -> Request {
        let params = Vec::new();

        Request {
            method: Method::Get,
            path: format!("/api/v4/spot/price_orders/{}", request.order_id).into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
