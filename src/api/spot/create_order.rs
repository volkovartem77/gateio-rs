use crate::http::{Credentials, Method, request::Request};
use serde_json::{Map, Value, json};

/// Request builder for creating trading orders.
///
/// Creates a new buy or sell order on Gate.io spot markets. Supports various order types
/// including limit, market, immediate-or-cancel (IOC), and fill-or-kill (FOK) orders.
///
/// # API Endpoint
/// `POST /api/v4/spot/orders`
///
/// # Authentication
/// This endpoint requires API key authentication with signing.
///
/// # Examples
///
/// ```rust,no_run
/// use gateio_rs::{
///     api::spot::create_order,
///     http::Credentials,
///     ureq::GateHttpClient,
/// };
///
/// let credentials = Credentials::new("api_key", "api_secret");
/// let client = GateHttpClient::default().credentials(credentials);
///
/// // Limit buy order
/// let request = create_order("BTC_USDT", "buy", "0.001")
///     .price("50000")
///     .order_type("limit")
///     .time_in_force("gtc")
///     .text("t-my-order-123");
/// let response = client.send(request)?;
///
/// // Market sell order
/// let request = create_order("BTC_USDT", "sell", "0.001")
///     .order_type("market");
/// let response = client.send(request)?;
/// # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
/// ```
///
/// # Parameter Details
///
/// ## Order Types (`order_type`)
/// - `"limit"`: Limit order (default) - requires price
/// - `"market"`: Market order - executes immediately at market price
/// - `"ioc"`: Immediate-or-cancel - taker only
/// - `"poc"`: Post-only - maker only (enjoys maker fee)
/// - `"fok"`: Fill-or-kill - complete fill or cancel
///
/// ## Time in Force (`time_in_force`)
/// - `"gtc"`: Good-till-cancelled (default)
/// - `"ioc"`: Immediate-or-cancel
/// - `"poc"`: Post-only
/// - `"fok"`: Fill-or-kill
///
/// ## Amount Rules
/// - **Limit orders**: Amount refers to base currency (e.g., BTC in BTC_USDT)
/// - **Market buy**: Amount refers to quote currency (e.g., USDT in BTC_USDT)
/// - **Market sell**: Amount refers to base currency (e.g., BTC in BTC_USDT)
///
/// ## Account Types (`account`)
/// - `"spot"`: Spot trading account
/// - `"margin"`: Margin trading account
/// - `"cross_margin"`: Cross margin account
/// - `"unified"`: Unified account
///
/// ## Self-Trading Prevention (`stp_act`)
/// - `"cn"`: Cancel newest orders
/// - `"co"`: Cancel oldest orders
/// - `"cb"`: Cancel both old and new orders
///
/// ## Text Field Rules
/// Custom order ID must:
/// - Be prefixed with `"t-"`
/// - Be no longer than 28 bytes (excluding prefix)
/// - Contain only: 0-9, A-Z, a-z, underscore, hyphen, or dot
pub struct CreateOrder {
    pub text: Option<String>,
    pub currency_pair: String,
    pub order_type: Option<String>,
    pub account: Option<String>,
    pub side: String,
    pub amount: String,
    pub price: Option<String>,
    pub time_in_force: Option<String>,
    pub iceberg: Option<String>,
    pub auto_borrow: Option<bool>,
    pub auto_repay: Option<bool>,
    pub stp_act: Option<String>,
    pub action_mode: Option<String>,
    pub x_gate_exp_time: Option<u128>,
    pub credentials: Option<Credentials>,
}

impl CreateOrder {
    pub fn new(currency_pair: &str, side: &str, amount: &str) -> Self {
        Self {
            text: None,
            currency_pair: currency_pair.to_owned(),
            order_type: None,
            account: None,
            side: side.to_owned(),
            amount: amount.to_owned(),
            price: None,
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

    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.into());
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
        self.auto_borrow = Some(auto_borrow.into());
        self
    }

    pub fn auto_repay(mut self, auto_repay: bool) -> Self {
        self.auto_repay = Some(auto_repay.into());
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

impl From<CreateOrder> for Request {
    fn from(request: CreateOrder) -> Request {
        let params = Vec::new();
        let mut payload = Map::new();

        payload.insert("currency_pair".to_string(), json!(request.currency_pair));
        payload.insert("side".to_string(), json!(request.side));
        payload.insert("amount".to_string(), json!(request.amount));

        if let Some(text) = request.text {
            payload.insert("text".to_string(), json!(text));
        }

        if let Some(order_type) = request.order_type {
            payload.insert("type".to_string(), json!(order_type));
        }

        if let Some(account) = request.account {
            payload.insert("account".to_string(), json!(account));
        }

        if let Some(price) = request.price {
            payload.insert("price".to_string(), json!(price));
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
            path: "/api/v4/spot/orders".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: request.x_gate_exp_time,
            credentials: request.credentials,
            sign: true,
        }
    }
}
