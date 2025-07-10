use super::order::Order;
use crate::http::{Credentials, Method, request::Request};

pub struct CreateBatchOrders {
    pub orders: Vec<Order>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CreateBatchOrders {
    pub fn new(orders: Vec<Order>) -> Self {
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
