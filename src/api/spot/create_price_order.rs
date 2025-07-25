use crate::http::{Credentials, Method, request::Request};
use serde_json::{Map, Value, json};

/// # SpotPriceTrigger
///
/// Trigger conditions for price-triggered orders
///
/// ##### price:
/// Trigger price that will activate the order
///
/// ##### rule:
/// Trigger rule:
/// - ">=" : Order triggers when market price is greater than or equal to trigger price
/// - "<=" : Order triggers when market price is less than or equal to trigger price
///
/// ##### expiration:
/// Valid duration in seconds (optional)
/// If not set, the order will remain active until manually cancelled
#[derive(Debug, Clone)]
pub struct SpotPriceTrigger {
    /// Trigger price that will activate the order
    pub price: String,
    /// Trigger rule ('>=' or '<=')
    pub rule: String,
    /// Valid duration in seconds (optional)
    pub expiration: Option<i64>,
}

impl SpotPriceTrigger {
    /// Create a new price trigger condition
    pub fn new(price: &str, rule: &str) -> Self {
        Self {
            price: price.to_owned(),
            rule: rule.to_owned(),
            expiration: None,
        }
    }

    /// Set the trigger expiration time in seconds
    pub fn expiration(mut self, expiration: i64) -> Self {
        self.expiration = Some(expiration);
        self
    }
}

/// # SpotPricePutOrder
///
/// The order to be placed when the trigger condition is met
///
/// ##### order_type:
/// Order type - currently only "limit" orders are supported for price-triggered orders
///
/// ##### side:
/// Order side:
/// - "buy" : Buy order
/// - "sell" : Sell order
///
/// ##### price:
/// Limit order price - the price at which the order will be placed when triggered
///
/// ##### amount:
/// Order amount - the quantity to trade
///
/// ##### account:
/// Trading account type:
/// - "normal" : Normal spot trading account
/// - "margin" : Margin trading account  
/// - "unified" : Unified trading account
///
/// ##### time_in_force:
/// Time in force for the triggered order:
/// - "gtc" : Good Till Cancelled (default)
/// - "ioc" : Immediate Or Cancel
/// - "fok" : Fill Or Kill
/// - "poc" : Post Only
#[derive(Debug, Clone)]
pub struct SpotPricePutOrder {
    /// Order type (currently only "limit" supported)
    pub order_type: String,
    /// Order side ("buy" or "sell")
    pub side: String,
    /// Limit order price
    pub price: String,
    /// Order amount/quantity
    pub amount: String,
    /// Trading account type
    pub account: Option<String>,
    /// Time in force for the triggered order
    pub time_in_force: Option<String>,
}

impl SpotPricePutOrder {
    /// Create a new order to place when triggered
    pub fn new(order_type: &str, side: &str, price: &str, amount: &str) -> Self {
        Self {
            order_type: order_type.to_owned(),
            side: side.to_owned(),
            price: price.to_owned(),
            amount: amount.to_owned(),
            account: None,
            time_in_force: None,
        }
    }

    /// Set the trading account type
    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.to_owned());
        self
    }

    /// Set the time in force for the triggered order
    pub fn time_in_force(mut self, time_in_force: &str) -> Self {
        self.time_in_force = Some(time_in_force.to_owned());
        self
    }
}

/// # Create a price-triggered order
///
/// A price-triggered order (also known as conditional order) will not enter the order book
/// until the trigger condition is met. Once triggered, it will attempt to place a limit order
/// at the preset price and amount.
///
/// ## Important Notes:
///
/// - The conditional order does not occupy your balance until it is triggered
/// - Make sure to set aside enough balance for this order
/// - A price condition order can only be triggered one time
/// - When using "<=", the trigger price should be less than the current market price  
/// - When using ">=", the trigger price should be greater than the current market price
/// - Only limit orders are supported as the triggered order type
///
/// ## Status Values:
/// - "open" : Waiting to trigger
/// - "cancelled" : Manually cancelled  
/// - "finish" : Successfully executed
/// - "failed" : Failed to execute
/// - "expired" : Expired
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#create-a-price-triggered-order)
pub struct CreatePriceOrder {
    /// Trigger conditions for the order
    pub trigger: SpotPriceTrigger,
    /// Order details to execute when triggered
    pub put: SpotPricePutOrder,
    /// Currency pair for the order
    pub market: String,
    /// Request expiration time in milliseconds
    pub x_gate_exp_time: Option<u128>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl CreatePriceOrder {
    /// Create a new price-triggered order request
    pub fn new(
        market: &str,
        trigger_price: &str,
        trigger_rule: &str,
        order_side: &str,
        order_price: &str,
        order_amount: &str,
    ) -> Self {
        Self {
            trigger: SpotPriceTrigger::new(trigger_price, trigger_rule),
            put: SpotPricePutOrder::new("limit", order_side, order_price, order_amount),
            market: market.to_owned(),
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    /// Set the trigger conditions
    pub fn trigger(mut self, trigger: SpotPriceTrigger) -> Self {
        self.trigger = trigger;
        self
    }

    /// Set the order details to execute when triggered
    pub fn put(mut self, put: SpotPricePutOrder) -> Self {
        self.put = put;
        self
    }

    /// Set the trigger expiration time in seconds
    pub fn trigger_expiration(mut self, expiration: i64) -> Self {
        self.trigger.expiration = Some(expiration);
        self
    }

    /// Set the trading account type for the triggered order
    pub fn account(mut self, account: &str) -> Self {
        self.put.account = Some(account.to_owned());
        self
    }

    /// Set the time in force for the triggered order
    pub fn time_in_force(mut self, time_in_force: &str) -> Self {
        self.put.time_in_force = Some(time_in_force.to_owned());
        self
    }

    /// Specify the expiration time (milliseconds);
    /// If the GATE receives the request time greater than the expiration time, the request will be rejected
    pub fn x_gate_exp_time(mut self, x_gate_exp_time: u128) -> Self {
        self.x_gate_exp_time = Some(x_gate_exp_time);
        self
    }

    /// Set API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CreatePriceOrder> for Request {
    fn from(request: CreatePriceOrder) -> Request {
        let params = Vec::new();
        let mut payload = Map::new();

        // Add market (currency pair)
        payload.insert("market".to_string(), json!(request.market));

        // Add trigger object
        let mut trigger_obj = Map::new();
        trigger_obj.insert("price".to_string(), json!(request.trigger.price));
        trigger_obj.insert("rule".to_string(), json!(request.trigger.rule));

        if let Some(expiration) = request.trigger.expiration {
            trigger_obj.insert("expiration".to_string(), json!(expiration));
        }

        payload.insert("trigger".to_string(), Value::Object(trigger_obj));

        // Add put object (the order to be placed when triggered)
        let mut put_obj = Map::new();
        put_obj.insert("type".to_string(), json!(request.put.order_type));
        put_obj.insert("side".to_string(), json!(request.put.side));
        put_obj.insert("price".to_string(), json!(request.put.price));
        put_obj.insert("amount".to_string(), json!(request.put.amount));

        if let Some(account) = request.put.account {
            put_obj.insert("account".to_string(), json!(account));
        }

        if let Some(time_in_force) = request.put.time_in_force {
            put_obj.insert("time_in_force".to_string(), json!(time_in_force));
        }

        payload.insert("put".to_string(), Value::Object(put_obj));

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
