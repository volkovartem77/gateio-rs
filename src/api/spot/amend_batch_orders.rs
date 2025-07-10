use crate::http::{Credentials, Method, request::Request};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OrderAmendment {
    pub order_id: String,
    pub currency_pair: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

impl OrderAmendment {
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

    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.to_owned());
        self
    }

    pub fn amount(mut self, amount: &str) -> Self {
        self.amount = Some(amount.to_owned());
        self
    }

    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.to_owned());
        self
    }

    pub fn amend_text(mut self, amend_text: &str) -> Self {
        self.amend_text = Some(amend_text.to_owned());
        self
    }
}

pub struct AmendBatchOrders {
    pub orders: Vec<OrderAmendment>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl AmendBatchOrders {
    pub fn new(orders: Vec<OrderAmendment>) -> Self {
        Self {
            orders,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time);
        self
    }

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
