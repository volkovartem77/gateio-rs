use crate::http::{Credentials, Method, request::Request};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CrossLiquidateOrder {
    pub currency_pair: String,
    pub amount: String,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_mode: Option<String>,
}

impl CrossLiquidateOrder {
    pub fn new(currency_pair: &str, amount: &str, price: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            amount: amount.to_owned(),
            price: price.to_owned(),
            text: None,
            action_mode: None,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_owned());
        self
    }

    pub fn action_mode(mut self, action_mode: &str) -> Self {
        self.action_mode = Some(action_mode.to_owned());
        self
    }
}

pub struct CreateCrossLiquidateOrders {
    pub orders: Vec<CrossLiquidateOrder>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CreateCrossLiquidateOrders {
    pub fn new(orders: Vec<CrossLiquidateOrder>) -> Self {
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

impl From<CreateCrossLiquidateOrders> for Request {
    fn from(request: CreateCrossLiquidateOrders) -> Request {
        let params = Vec::new();
        let payload = serde_json::to_string(&request.orders).unwrap();

        Request {
            method: Method::Post,
            path: "/api/v4/spot/cross_liquidate_orders".into(),
            params,
            payload,
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
