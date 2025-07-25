use serde::Serialize;

/// Order data structure for creating and managing spot orders
#[derive(Debug, Clone, Serialize)]
pub struct Order {
    /// User-defined text information for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Currency pair for the order (e.g., "BTC_USDT")
    pub currency_pair: String,
    /// Order type ("limit", "market", "immediate_or_cancel", "fill_or_kill")
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Account type ("spot", "margin", "cross_margin")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Order side ("buy" or "sell")
    pub side: String,
    /// Order amount in base currency
    pub amount: String,
    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force policy ("gtc", "ioc", "fok")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    /// Iceberg order visible amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<String>,
    /// Whether to automatically borrow for margin trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    /// Whether to automatically repay for margin trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<bool>,
    /// Self-trade prevention action ("cn", "co", "cb")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
    /// Action mode for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_mode: Option<String>,
}

impl Order {
    /// Creates a new Order with required parameters
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
        }
    }

    /// Sets user-defined text information for the order
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the order type
    pub fn order_type(mut self, order_type: &str) -> Self {
        self.order_type = Some(order_type.into());
        self
    }

    /// Sets the account type for the order
    pub fn account(mut self, account: &str) -> Self {
        self.account = Some(account.into());
        self
    }

    /// Sets the order price (required for limit orders)
    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.into());
        self
    }

    /// Sets the time in force policy
    pub fn time_in_force(mut self, time_in_force: &str) -> Self {
        self.time_in_force = Some(time_in_force.into());
        self
    }

    /// Sets the iceberg order visible amount
    pub fn iceberg(mut self, iceberg: &str) -> Self {
        self.iceberg = Some(iceberg.into());
        self
    }

    /// Sets whether to automatically borrow for margin trading
    pub fn auto_borrow(mut self, auto_borrow: bool) -> Self {
        self.auto_borrow = Some(auto_borrow);
        self
    }

    /// Sets whether to automatically repay for margin trading
    pub fn auto_repay(mut self, auto_repay: bool) -> Self {
        self.auto_repay = Some(auto_repay);
        self
    }

    /// Sets the self-trade prevention action
    pub fn stp_act(mut self, stp_act: &str) -> Self {
        self.stp_act = Some(stp_act.into());
        self
    }

    /// Sets the action mode for the order
    pub fn action_mode(mut self, action_mode: &str) -> Self {
        self.action_mode = Some(action_mode.into());
        self
    }
}
