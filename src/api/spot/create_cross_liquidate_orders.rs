use crate::http::{Credentials, Method, request::Request};
use serde::Serialize;

/// # Cross liquidate order
///
/// Represents a single cross-liquidation order for margin trading.
/// Cross liquidation allows closing positions across different currency pairs
/// to meet margin requirements.
#[derive(Debug, Clone, Serialize)]
pub struct CrossLiquidateOrder {
    /// Currency pair for the order
    pub currency_pair: String,
    /// Order amount
    pub amount: String,
    /// Order price
    pub price: String,
    /// Custom order text/label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Processing mode for the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_mode: Option<String>,
}

impl CrossLiquidateOrder {
    /// Create a new cross liquidate order
    pub fn new(currency_pair: &str, amount: &str, price: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            amount: amount.to_owned(),
            price: price.to_owned(),
            text: None,
            action_mode: None,
        }
    }

    /// Set custom order text/label
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_owned());
        self
    }

    /// Set the processing mode for the response
    pub fn action_mode(mut self, action_mode: &str) -> Self {
        self.action_mode = Some(action_mode.to_owned());
        self
    }
}

/// # Create cross liquidate orders
///
/// Create multiple cross-liquidation orders for margin trading.
/// Cross liquidation helps close positions across different currency pairs
/// to meet margin requirements automatically.
///
/// ## Important Notes:
/// - Only available for margin trading accounts
/// - Orders are processed to reduce overall margin risk
/// - All orders must be valid for the request to succeed
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#place-cross-liquidation-order)
pub struct CreateCrossLiquidateOrders {
    /// List of cross liquidate orders to create
    pub orders: Vec<CrossLiquidateOrder>,
    /// Request expiration time in milliseconds
    pub x_gate_exp_time: Option<u128>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl CreateCrossLiquidateOrders {
    /// Create a new cross liquidate orders request
    pub fn new(orders: Vec<CrossLiquidateOrder>) -> Self {
        Self {
            orders,
            x_gate_exp_time: None,
            credentials: None,
        }
    }

    /// Set the request expiration time in milliseconds
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
