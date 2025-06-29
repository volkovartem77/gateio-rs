pub mod get_currencies;
pub mod get_currency;
pub mod get_currency_pairs;
pub mod get_currency_pair;
pub mod get_ticker;
pub mod get_orderbook;
pub mod get_market_trades;
pub mod get_candlesticks;
pub mod get_batch_user_fee;
pub mod get_account;
pub mod get_account_book;
pub mod create_order;
pub mod cancel_order;
pub mod amend_order;
pub mod order;
pub mod create_batch_orders;
pub mod get_open_orders;
pub mod get_orders;
pub mod get_order;
pub mod cancel_batch_orders;
pub mod get_my_trades;
pub mod get_fee;
pub mod create_price_order;
pub mod get_price_orders;
pub mod cancel_all_price_orders;
pub mod get_price_order;
pub mod cancel_price_order;

use get_currencies::GetCurrencies;
use get_currency::GetCurrency;
use get_currency_pairs::GetCurrencyPairs;
use get_currency_pair::GetCurrencyPair;
use get_ticker::GetTicker;
use get_orderbook::GetOrderbook;
use get_market_trades::GetMarketTrades;
use get_candlesticks::GetCandlesticks;
use get_batch_user_fee::GetBatchUserFee;
use get_account::GetAccount;
use get_account_book::GetAccountBook;

use create_order::CreateOrder;
use cancel_order::CancelOrder;
use amend_order::AmendOrder;
pub use order::Order;
pub use cancel_batch_orders::CancelOrderRequest;
use create_batch_orders::CreateBatchOrders;
use get_open_orders::GetOpenOrders;
use get_orders::GetOrders;
use get_order::GetOrder;
use cancel_batch_orders::CancelBatchOrders;
use get_my_trades::GetMyTrades;
use get_fee::GetFee;
use create_price_order::CreatePriceOrder;
use get_price_orders::GetPriceOrders;
use cancel_all_price_orders::CancelAllPriceOrders;
use get_price_order::GetPriceOrder;
use cancel_price_order::CancelPriceOrder;

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

/// Retrieve market trades
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#retrieve-market-trades)
pub fn get_market_trades(currency_pair: &str) -> GetMarketTrades {
    GetMarketTrades::new(currency_pair)
}

/// Market candlesticks
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#market-candlesticks)
pub fn get_candlesticks(currency_pair: &str) -> GetCandlesticks {
    GetCandlesticks::new(currency_pair)
}

/// Query a batch of user trading fee rates
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#query-a-batch-of-user-trading-fee-rates)
pub fn get_batch_user_fee(currency_pairs: &str) -> GetBatchUserFee {
    GetBatchUserFee::new(currency_pairs)
}

/// List spot accounts <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#list-spot-accounts)
pub fn get_account() -> GetAccount {
    GetAccount::new()
}

/// Query account book <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#query-account-book)
pub fn get_account_book() -> GetAccountBook {
    GetAccountBook::new()
}

/// List all open orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#list-all-open-orders)
pub fn get_open_orders() -> GetOpenOrders {
    GetOpenOrders::new()
}

/// Create an order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#create-an-order)
pub fn create_order(currency_pair: &str, side: &str, amount: &str) -> CreateOrder {
    CreateOrder::new(currency_pair, side, amount)
}

/// List orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#list-orders)
pub fn get_orders() -> GetOrders {
    GetOrders::new()
}

/// Get a single order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#get-a-single-order)
pub fn get_order(order_id: &str, currency_pair: &str) -> GetOrder {
    GetOrder::new(order_id, currency_pair)
}

/// Cancel a batch of orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-a-batch-of-orders-with-an-id-list)
pub fn cancel_batch_orders(orders: Vec<CancelOrderRequest>) -> CancelBatchOrders {
    CancelBatchOrders::new(orders)
}

/// List personal trading history <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#list-personal-trading-history)
pub fn get_my_trades() -> GetMyTrades {
    GetMyTrades::new()
}

/// Query user trading fee rates <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#query-user-trading-fee-rates)
pub fn get_fee() -> GetFee {
    GetFee::new()
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

/// Create a batch of orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#create-a-batch-of-orders)
pub fn create_batch_orders(orders: Vec<Order>) -> CreateBatchOrders {
    CreateBatchOrders::new(orders)
}

/// Create a price-triggered order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#create-a-price-triggered-order)
pub fn create_price_order(
    market: &str,
    trigger_price: &str, 
    trigger_rule: &str,
    order_side: &str,
    order_price: &str,
    order_amount: &str,
) -> CreatePriceOrder {
    CreatePriceOrder::new(market, trigger_price, trigger_rule, order_side, order_price, order_amount)
}

/// Retrieve running auto order list <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#retrieve-running-auto-order-list)
pub fn get_price_orders() -> GetPriceOrders {
    GetPriceOrders::new()
}

/// Cancel all price-triggered orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-all-price-triggered-orders)
pub fn cancel_all_price_orders() -> CancelAllPriceOrders {
    CancelAllPriceOrders::new()
}

/// Get a price-triggered order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#get-a-price-triggered-order)
pub fn get_price_order(order_id: &str) -> GetPriceOrder {
    GetPriceOrder::new(order_id)
}

/// Cancel a price-triggered order <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-a-price-triggered-order)
pub fn cancel_price_order(order_id: &str) -> CancelPriceOrder {
    CancelPriceOrder::new(order_id)
}

