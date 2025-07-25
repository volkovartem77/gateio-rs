use crate::http::{Credentials, Method, request::Request};

/// # Cancel a spot order
///
/// Cancel a specific spot order by order ID or custom text field.
///
/// ## Order ID
/// The order ID returned when the order was successfully created or the custom ID
/// specified by the user's creation (i.e. the text field).
/// Operations based on custom IDs can only be checked in pending orders.
/// Only order ID can be used after the order is finished (transaction/cancel)
///
/// ## Action Mode
/// Processing Mode - When placing an order, different fields are returned based on the action_mode
/// - ACK: Asynchronous mode, returns only key order fields
/// - RESULT: No clearing information  
/// - FULL: Full mode (default)
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-a-single-order)
pub struct CancelOrder {
    /// Order ID or custom text field
    pub order_id: String,
    /// Currency pair for the order
    pub currency_pair: String,
    /// Trading account type
    pub account: Option<String>,
    /// Processing mode for the response
    pub action_mode: Option<String>,
    /// Request expiration time in milliseconds
    pub x_gate_exp_time: Option<u128>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl CancelOrder {
    /// Create a new cancel order request
    pub fn new(order_id: &str, currency_pair: &str) -> Self {
        Self {
            order_id: order_id.to_owned(),
            currency_pair: currency_pair.to_owned(),
            account: None,
            action_mode: None,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    /// Set the trading account type
    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    /// Set the processing mode for the response
    pub fn action_mode(mut self, action_mode: &str) -> Self {
        self.action_mode = Some(action_mode.into());
        self
    }

    /// Specify the expiration time (milliseconds);<br/>
    /// If the GATE receives the request time greater than the expiration time, the request will be rejected
    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time.into());
        self
    }

    /// Set API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CancelOrder> for Request {
    fn from(request: CancelOrder) -> Request {
        let mut params = vec![("currency_pair".to_owned(), request.currency_pair)];

        if let Some(account) = request.account {
            params.push(("account".into(), account.to_string()));
        }

        if let Some(action_mode) = request.action_mode {
            params.push(("action_mode".into(), action_mode.to_string()));
        }

        Request {
            method: Method::Delete,
            path: format!("/api/v4/spot/orders/{}", request.order_id).into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
