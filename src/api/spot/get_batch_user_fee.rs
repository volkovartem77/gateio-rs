use serde_json::{Map, Value};
use crate::http::{request::Request, Credentials, Method};

pub struct GetBatchUserFee {
    pub currency_pairs: String,
    pub credentials: Option<Credentials>,
}

impl GetBatchUserFee {
    pub fn new(currency_pairs: &str) -> Self {
        Self {
            currency_pairs: currency_pairs.to_owned(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetBatchUserFee> for Request {
    fn from(request: GetBatchUserFee) -> Request {
        let mut params = vec![
            ("currency_pairs".to_owned(), request.currency_pairs),
        ];

        // TODO: делать этот файл
        //  идея в том что бы сделать объект Order (api/spot/order.rs)
        //  и юзер может создавать свои ордера
        //  а эта функция будет принимать на вход Vec<Order> и собирать payload
        //  и надо затестить это все

        let payload = Map::new();

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Get,
            path: "/api/v4/spot/batch_fee".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}