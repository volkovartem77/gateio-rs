use crate::http::{Credentials, Method, request::Request};
use serde::Serialize;

/// Single order amendment parameters
#[derive(Debug, Clone, Serialize)]
pub struct OrderAmendment {
    /// Order ID to amend
    pub order_id: String,
    /// Trading pair
    pub currency_pair: String,
    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Order amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Custom amendment text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

impl OrderAmendment {
    /// Create new order amendment
    pub fn new(order_id: &str, currency_pair: &str) -> Self {
        Self {
            order_id: order_id.to_owned(),
            currency_pair: currency_pair.to_owned(),
            account: None,
            amount: None,
            price: None,
            amend_text: None,
        }
    }

    /// Set account type
    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.to_owned());
        self
    }

    /// Set order amount
    pub fn amount(mut self, amount: &str) -> Self {
        self.amount = Some(amount.to_owned());
        self
    }

    /// Set order price
    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.to_owned());
        self
    }

    /// Set amendment text
    pub fn amend_text(mut self, amend_text: &str) -> Self {
        self.amend_text = Some(amend_text.to_owned());
        self
    }
}

/// Batch order amendment request
pub struct AmendBatchOrders {
    /// Orders to amend
    pub orders: Vec<OrderAmendment>,
    /// Request expiration time
    pub x_gate_exp_time: Option<u128>,
    /// API credentials
    pub credentials: Option<Credentials>,
}

impl AmendBatchOrders {
    /// Create batch amendment request
    pub fn new(orders: Vec<OrderAmendment>) -> Self {
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

impl From<AmendBatchOrders> for Request {
    fn from(request: AmendBatchOrders) -> Request {
        let params = Vec::new();
        let payload = serde_json::to_string(&request.orders).unwrap();

        Request {
            method: Method::Post,
            path: "/api/v4/spot/amend_batch_orders".into(),
            params,
            payload,
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
