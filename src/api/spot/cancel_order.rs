use serde_json::{Map, Value};
use crate::http::{request::Request, Credentials, Method};

/// # Detailed descriptions
/// ##### order_id:
/// The order ID returned when the order was successfully created or the custom ID
/// specified by the user's creation (i.e. the text field). <br/>
/// Operations based on custom IDs can only be checked in pending orders.
/// Only order ID can be used after the order is finished (transaction/cancel)
///
/// ##### action_mode:
/// Processing Mode <br/>
/// When placing an order, different fields are returned based on the action_mode
/// - ACK: Asynchronous mode, returns only key order fields
/// - RESULT: No clearing information
/// - FULL: Full mode (default)

pub struct CancelOrder {
    pub order_id: String,
    pub currency_pair: String,
    pub account: Option<String>,
    pub action_mode: Option<String>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CancelOrder {
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

    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

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

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CancelOrder> for Request {
    fn from(request: CancelOrder) -> Request {
        let mut params = vec![
            ("currency_pair".to_owned(), request.currency_pair),
        ];

        if let Some(account) = request.account {
            params.push(("account".into(), account.to_string()));
        }

        if let Some(action_mode) = request.action_mode {
            params.push(("action_mode".into(), action_mode.to_string()));
        }

        let payload = Map::new();

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Delete,
            path: format!("/api/v4/spot/orders/{}", request.order_id).into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}