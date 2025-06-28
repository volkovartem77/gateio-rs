use crate::http::{request::Request, Credentials, Method};
use serde::Serialize;

#[derive(Serialize)]
pub struct CancelOrderRequest {
    pub id: String,
    pub currency_pair: String,
}

pub struct CancelBatchOrders {
    pub orders: Vec<CancelOrderRequest>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CancelBatchOrders {
    pub fn new(orders: Vec<CancelOrderRequest>) -> Self {
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