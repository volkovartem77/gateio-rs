use crate::http::{Credentials, Method, request::Request};
use serde_json::{Map, Value, json};

/// # Countdown cancel all orders
///
/// Start a countdown timer to cancel all open spot orders.
/// If the timeout is reached without being reset, all open orders will be cancelled automatically.
/// This is useful as a safety mechanism to prevent orders from remaining open if connection is lost.
///
/// ## Important Notes:
/// - The countdown can be reset by calling this endpoint again with a new timeout
/// - Setting timeout to 0 will disable the countdown
/// - Only affects orders for the specified currency pair (if provided)
/// - Only affects spot orders, not futures or other types
///
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#countdown-cancel-orders)
pub struct CountdownCancelAll {
    /// Countdown timeout in seconds (0 to disable)
    pub timeout: i64,
    /// Optional currency pair to limit cancellation to
    pub currency_pair: Option<String>,
    /// API credentials for authentication
    pub credentials: Option<Credentials>,
}

impl CountdownCancelAll {
    /// Create a new countdown cancel all request
    pub fn new(timeout: i64) -> Self {
        Self {
            timeout,
            currency_pair: None,
            credentials: None,
        }
    }

    /// Set the currency pair to limit cancellation to
    pub fn currency_pair(mut self, currency_pair: &str) -> Self {
        self.currency_pair = Some(currency_pair.to_string());
        self
    }

    /// Set API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<CountdownCancelAll> for Request {
    fn from(request: CountdownCancelAll) -> Request {
        let params = Vec::new();
        let mut payload = Map::new();

        payload.insert("timeout".to_string(), json!(request.timeout));

        if let Some(currency_pair) = request.currency_pair {
            payload.insert("currency_pair".to_string(), json!(currency_pair));
        }

        let payload_json = Value::Object(payload);

        Request {
            method: Method::Post,
            path: "/api/v4/spot/countdown_cancel_all".into(),
            params,
            payload: payload_json.to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: true,
        }
    }
}
