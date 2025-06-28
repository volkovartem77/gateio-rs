use serde_json::{json, Map, Value};
use crate::http::{request::Request, Credentials, Method};

/// Create a price-triggered order
/// 
/// Price-triggered orders are conditional orders that will be executed when 
/// the market price reaches the specified trigger price.
pub struct CreatePriceOrder {
    pub text: Option<String>,
    pub currency_pair: String,
    pub trigger_price: String,
    pub side: String,
    pub amount: String,
    pub order_type: Option<String>,
    pub price: Option<String>,
    pub account: Option<String>,
    pub time_in_force: Option<String>,
    pub iceberg: Option<String>,
    pub auto_borrow: Option<bool>,
    pub auto_repay: Option<bool>,
    pub stp_act: Option<String>,
    pub action_mode: Option<String>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CreatePriceOrder {
    pub fn new(currency_pair: &str, trigger_price: &str, side: &str, amount: &str) -> Self {
        Self {
            text: None,
            currency_pair: currency_pair.to_owned(),
            trigger_price: trigger_price.to_owned(),
            side: side.to_owned(),
            amount: amount.to_owned(),
            order_type: None,
            price: None,
            account: None,
            time_in_force: None,
            iceberg: None,
            auto_borrow: None,
            auto_repay: None,
            stp_act: None,
            action_mode: None,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn order_type(mut self, order_type: &str) -> Self {
        self.order_type = Some(order_type.into());
        self
    }

    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    pub fn time_in_force(mut self, time_in_force: &str) -> Self {
        self.time_in_force = Some(time_in_force.into());
        self
    }

    pub fn iceberg(mut self, iceberg: &str) -> Self {
        self.iceberg = Some(iceberg.into());
        self
    }

    pub fn auto_borrow(mut self, auto_borrow: bool) -> Self {
        self.auto_borrow = Some(auto_borrow);
        self
    }

    pub fn auto_repay(mut self, auto_repay: bool) -> Self {
        self.auto_repay = Some(auto_repay);
        self
    }

    pub fn stp_act(mut self, stp_act: &str) -> Self {
        self.stp_act = Some(stp_act.into());
        self
    }

    pub fn action_mode(mut self, action_mode: &str) -> Self {
        self.action_mode = Some(action_mode.into());
        self
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

impl From<CreatePriceOrder> for Request {
    fn from(request: CreatePriceOrder) -> Request {
        let mut params = Vec::new();
        let mut payload = Map::new();

        payload.insert("currency_pair".to_string(), json!(request.currency_pair));
        payload.insert("trigger_price".to_string(), json!(request.trigger_price));
        payload.insert("side".to_string(), json!(request.side));
        payload.insert("amount".to_string(), json!(request.amount));

        if let Some(text) = request.text {
            payload.insert("text".to_string(), json!(text));
        }

        if let Some(order_type) = request.order_type {
            payload.insert("order_type".to_string(), json!(order_type));
        }

        if let Some(price) = request.price {
            payload.insert("price".to_string(), json!(price));
        }

        if let Some(account) = request.account {
            payload.insert("account".to_string(), json!(account));
        }

        if let Some(time_in_force) = request.time_in_force {
            payload.insert("time_in_force".to_string(), json!(time_in_force));
        }

        if let Some(iceberg) = request.iceberg {
            payload.insert("iceberg".to_string(), json!(iceberg));
        }

        if let Some(auto_borrow) = request.auto_borrow {
            payload.insert("auto_borrow".to_string(), json!(auto_borrow));
        }

        if let Some(auto_repay) = request.auto_repay {
            payload.insert("auto_repay".to_string(), json!(auto_repay));
        }

        if let Some(stp_act) = request.stp_act {
            payload.insert("stp_act".to_string(), json!(stp_act));
        }

        if let Some(action_mode) = request.action_mode {
            payload.insert("action_mode".to_string(), json!(action_mode));
        }

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Post,
            path: "/api/v4/spot/price_orders".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}