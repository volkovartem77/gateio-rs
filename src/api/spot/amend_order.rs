use crate::http::{Credentials, Method, request::Request};
use serde_json::{Map, Value, json};

/// Order amendment request
pub struct AmendOrder {
    /// Order ID to amend
    pub order_id: String,
    /// Trading pair
    pub currency_pair: String,
    /// Account type
    pub account: Option<String>,
    /// Order amount
    pub amount: Option<String>,
    /// Order price
    pub price: Option<String>,
    /// Custom amendment text
    pub amend_text: Option<String>,
    /// Processing mode
    pub action_mode: Option<String>,
    /// Request expiration time
    pub x_gate_exp_time: Option<u128>,
    /// API credentials
    pub credentials: Option<Credentials>,
}

impl AmendOrder {
    /// Create order amendment request
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

    /// Set account type
    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    /// Set order amount
    pub fn amount(mut self, amount: &str) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Set order price
    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.into());
        self
    }

    /// Set amendment text
    pub fn amend_text(mut self, amend_text: &str) -> Self {
        self.amend_text = Some(amend_text.into());
        self
    }

    /// Set processing mode
    pub fn action_mode(mut self, action_mode: &str) -> Self {
        self.action_mode = Some(action_mode.into());
        self
    }

    /// Set expiration time
    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time.into());
        self
    }

    /// Set API credentials
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<AmendOrder> for Request {
    fn from(request: AmendOrder) -> Request {
        let params = Vec::new();
        let mut payload = Map::new();

        payload.insert("currency_pair".to_string(), json!(request.currency_pair));

        if let Some(account) = request.account {
            payload.insert("account".to_string(), json!(account));
        }

        if let Some(amount) = request.amount {
            payload.insert("amount".to_string(), json!(amount));
        }

        if let Some(price) = request.price {
            payload.insert("price".to_string(), json!(price));
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
