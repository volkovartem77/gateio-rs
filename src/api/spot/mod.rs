pub mod get_ticker;
pub mod get_account;
pub mod create_order;
pub mod get_currency_pairs;
pub mod get_currency_pair;
pub mod cancel_order;
pub mod amend_order;
pub mod get_currencies;
pub mod get_currency;
pub mod get_orderbook;

use get_currencies::GetCurrencies;
use get_currency::GetCurrency;
use get_currency_pairs::GetCurrencyPairs;
use get_currency_pair::GetCurrencyPair;
use get_orderbook::GetOrderbook;
use get_ticker::GetTicker;
use get_account::GetAccount;
use create_order::CreateOrder;
use cancel_order::CancelOrder;
use amend_order::AmendOrder;

/// List all currencies' details <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#list-all-currencies-details)
pub fn get_currencies() -> GetCurrencies {
    GetCurrencies::new()
}

/// Get details of a specific currency <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#get-details-of-a-specific-currency)
pub fn get_currency(currency: &str) -> GetCurrency {
    GetCurrency::new(currency)
}

/// List all currency pairs supported <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#list-all-currency-pairs-supported)
pub fn get_currency_pairs() -> GetCurrencyPairs {
    GetCurrencyPairs::new()
}

/// Get details of a specific currency pair <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#get-details-of-a-specifc-currency-pair)
pub fn get_currency_pair(currency_pair: &str) -> GetCurrencyPair {
    GetCurrencyPair::new(currency_pair)
}

/// Retrieve ticker information <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#retrieve-ticker-information)
pub fn get_ticker() -> GetTicker {
    GetTicker::new()
}

/// Retrieve order book <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#retrieve-order-book)
pub fn get_orderbook(currency_pair: &str) -> GetOrderbook {
    GetOrderbook::new(currency_pair)
}

/// List spot accounts <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#list-spot-accounts)
pub fn get_account() -> GetAccount {
    GetAccount::new()
}

/// Create an order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#create-an-order)
pub fn create_order(currency_pair: &str, side: &str, amount: &str) -> CreateOrder {
    CreateOrder::new(currency_pair, side, amount)
}

/// Cancel a single order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-a-single-order)
pub fn cancel_order(order_id: &str, currency_pair: &str) -> CancelOrder {
    CancelOrder::new(order_id, currency_pair)
}

/// Amend an order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#amend-an-order)
pub fn amend_order(order_id: &str, currency_pair: &str) -> AmendOrder {
    AmendOrder::new(order_id, currency_pair)
}

