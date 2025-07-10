use crate::http::{Credentials, Method, request::Request};
use serde_json::{Map, Value, json};

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
///
pub struct AmendOrder {
    pub order_id: String,
    pub currency_pair: String,
    pub account: Option<String>,
    pub amount: Option<String>,
    pub price: Option<String>,
    pub amend_text: Option<String>,
    pub action_mode: Option<String>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl AmendOrder {
    pub fn new(order_id: &str, currency_pair: &str) -> Self {
        Self {
            order_id: order_id.to_owned(),
            currency_pair: currency_pair.to_owned(),
            account: None,
            amount: None,
            price: None,
            amend_text: None,
            action_mode: None,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    pub fn amount(mut self, amount: &str) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn amend_text(mut self, amend_text: &str) -> Self {
        self.amend_text = Some(amend_text.into());
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

impl From<AmendOrder> for Request {
    fn from(request: AmendOrder) -> Request {
        let mut params = Vec::new();
        let mut payload = Map::new();

        payload.insert("currency_pair".to_string(), json!(request.currency_pair));

        if let Some(account) = request.account {
            payload.insert("account".to_string(), json!(account));
        }

        if let Some(amount) = request.amount {
            payload.insert("amount".to_string(), json!(amount));
        }

        if let Some(amend_text) = request.amend_text {
            payload.insert("amend_text".to_string(), json!(amend_text));
        }

        if let Some(action_mode) = request.action_mode {
            payload.insert("action_mode".to_string(), json!(action_mode));
        }

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Patch,
            path: format!("/api/v4/spot/orders/{}", request.order_id).into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
