use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub currency_pair: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    pub side: String,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_mode: Option<String>,
}

impl Order {
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
}
