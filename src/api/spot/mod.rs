//! # Gate.io Spot Trading API
//!
//! This module provides a comprehensive interface to Gate.io's Spot trading API endpoints.
//! All functions return request builders that can be configured with additional parameters
//! before sending via the HTTP client.
//!
//! ## Categories
//!
//! ### Market Data (Public)
//! - [`get_currencies`] - List all supported currencies
//! - [`get_currency`] - Get specific currency details
//! - [`get_currency_pairs`] - List all trading pairs
//! - [`get_currency_pair`] - Get specific pair details
//! - [`get_ticker`] - Get ticker information
//! - [`get_orderbook`] - Get order book data
//! - [`get_market_trades`] - Get recent market trades
//! - [`get_candlesticks`] - Get historical price data
//! - [`get_server_time`] - Get server timestamp
//!
//! ### Account Management (Private)
//! - [`get_account`] - List spot account balances
//! - [`get_account_book`] - Query account transaction history
//! - [`get_fee`] - Get trading fee rates
//! - [`get_batch_user_fee`] - Get fee rates for multiple pairs
//!
//! ### Order Management (Private)
//! - [`create_order`] - Place a new order
//! - [`create_batch_orders`] - Place multiple orders
//! - [`get_orders`] - List order history
//! - [`get_order`] - Get specific order details
//! - [`get_open_orders`] - List active orders
//! - [`amend_order`] - Modify an existing order
//! - [`amend_batch_orders`] - Modify multiple orders
//! - [`cancel_order`] - Cancel a specific order
//! - [`cancel_batch_orders`] - Cancel multiple orders
//! - [`cancel_all_open_orders`] - Cancel all open orders
//! - [`countdown_cancel_all`] - Set auto-cancel timer
//!
//! ### Advanced Orders (Private)
//! - [`create_price_order`] - Create stop/trigger orders
//! - [`get_price_orders`] - List active trigger orders
//! - [`get_price_order`] - Get specific trigger order
//! - [`cancel_price_order`] - Cancel trigger order
//! - [`cancel_all_price_orders`] - Cancel all trigger orders
//! - [`create_cross_liquidate_orders`] - Cross-currency liquidation
//!
//! ### Trading History (Private)
//! - [`get_my_trades`] - Get personal trade history
//! - [`get_insurance_history`] - Get insurance fund data
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use gateio_rs::{
//!     api::spot::{get_ticker, create_order},
//!     http::Credentials,
//!     ureq::GateHttpClient,
//! };
//!
//! // Public API - no authentication needed
//! let client = GateHttpClient::default();
//! let ticker_req = get_ticker().currency_pair("BTC_USDT");
//! let response = client.send(ticker_req)?;
//!
//! // Private API - requires authentication
//! let credentials = Credentials::new("api_key", "api_secret");
//! let client = GateHttpClient::default().credentials(credentials);
//! let order_req = create_order("BTC_USDT", "buy", "0.001").price("50000");
//! let response = client.send(order_req)?;
//! # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
//! ```
//!
//! For detailed parameter documentation, see the [Gate.io API Documentation](https://www.gate.com/docs/developers/apiv4/#spot).

pub mod amend_batch_orders;
pub mod amend_order;
pub mod cancel_all_open_orders;
pub mod cancel_all_price_orders;
pub mod cancel_batch_orders;
pub mod cancel_order;
pub mod cancel_price_order;
pub mod countdown_cancel_all;
pub mod create_batch_orders;
pub mod create_cross_liquidate_orders;
pub mod create_order;
pub mod create_price_order;
pub mod get_account;
pub mod get_account_book;
pub mod get_batch_user_fee;
pub mod get_candlesticks;
pub mod get_currencies;
pub mod get_currency;
pub mod get_currency_pair;
pub mod get_currency_pairs;
pub mod get_fee;
pub mod get_insurance_history;
pub mod get_market_trades;
pub mod get_my_trades;
pub mod get_open_orders;
pub mod get_order;
pub mod get_orderbook;
pub mod get_orders;
pub mod get_price_order;
pub mod get_price_orders;
pub mod get_server_time;
pub mod get_ticker;
pub mod order;

use get_account::GetAccount;
use get_account_book::GetAccountBook;
use get_batch_user_fee::GetBatchUserFee;
use get_candlesticks::GetCandlesticks;
use get_currencies::GetCurrencies;
use get_currency::GetCurrency;
use get_currency_pair::GetCurrencyPair;
use get_currency_pairs::GetCurrencyPairs;
use get_market_trades::GetMarketTrades;
use get_orderbook::GetOrderbook;
use get_ticker::GetTicker;

use amend_batch_orders::AmendBatchOrders;
pub use amend_batch_orders::OrderAmendment;
use amend_order::AmendOrder;
use cancel_all_open_orders::CancelAllOpenOrders;
use cancel_all_price_orders::CancelAllPriceOrders;
use cancel_batch_orders::CancelBatchOrders;
pub use cancel_batch_orders::CancelOrderRequest;
use cancel_order::CancelOrder;
use cancel_price_order::CancelPriceOrder;
use countdown_cancel_all::CountdownCancelAll;
use create_batch_orders::CreateBatchOrders;
use create_cross_liquidate_orders::CreateCrossLiquidateOrders;
pub use create_cross_liquidate_orders::CrossLiquidateOrder;
use create_order::CreateOrder;
use create_price_order::CreatePriceOrder;
use get_fee::GetFee;
use get_insurance_history::GetInsuranceHistory;
use get_my_trades::GetMyTrades;
use get_open_orders::GetOpenOrders;
use get_order::GetOrder;
use get_orders::GetOrders;
use get_price_order::GetPriceOrder;
use get_price_orders::GetPriceOrders;
use get_server_time::GetServerTime;
pub use order::Order;

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

/// Retrieve ticker information for currency pairs.
///
/// Returns 24hr trading statistics including price, volume, and changes.
/// Can be used to get data for a specific pair or all pairs at once.
///
/// # Parameters
/// - `currency_pair`: Optional. Specific trading pair (e.g., "BTC_USDT")
/// - `timezone`: Optional. Timezone for calculation ("utc0" to "utc12", "utc-12" to "utc-1")
///
/// # Examples
///
/// ```rust,no_run
/// use gateio_rs::{api::spot::get_ticker, ureq::GateHttpClient};
///
/// let client = GateHttpClient::default();
///
/// // Get ticker for specific pair
/// let request = get_ticker().currency_pair("BTC_USDT");
/// let response = client.send(request)?;
///
/// // Get all tickers
/// let request = get_ticker();
/// let response = client.send(request)?;
/// # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
/// ```
///
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

/// List spot account balances and information.
///
/// Returns account balances for all currencies in the spot trading account.
/// Shows available balance, locked balance, and total balance for each currency.
///
/// # Authentication
/// This endpoint requires API key authentication.
///
/// # Optional Parameters (via builder methods)
/// - `currency`: Filter by specific currency (e.g., "BTC", "USDT")
///
/// # Examples
///
/// ```rust,no_run
/// use gateio_rs::{
///     api::spot::get_account,
///     http::Credentials,
///     ureq::GateHttpClient,
/// };
///
/// let credentials = Credentials::new("api_key", "api_secret");
/// let client = GateHttpClient::default().credentials(credentials);
///
/// // Get all account balances
/// let request = get_account();
/// let response = client.send(request)?;
///
/// // Get specific currency balance
/// let request = get_account().currency("BTC");
/// let response = client.send(request)?;
/// # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
/// ```
///
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

/// Create a new trading order.
///
/// Places a buy or sell order on the specified trading pair. Supports various order types
/// including market, limit, and immediate-or-cancel orders.
///
/// # Parameters
/// - `currency_pair`: Trading pair (e.g., "BTC_USDT")
/// - `side`: Order side ("buy" or "sell")
/// - `amount`: Order amount in base currency
///
/// # Optional Parameters (via builder methods)
/// - `price`: Order price (required for limit orders)
/// - `order_type`: "limit", "market", "ioc", "poc", "fok" (default: "limit")
/// - `account`: Account type ("spot", "margin", "cross_margin", "unified")
/// - `time_in_force`: "gtc", "ioc", "poc", "fok" (default: "gtc")
/// - `text`: Custom order ID for identification
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
///     .time_in_force("gtc");
/// let response = client.send(request)?;
///
/// // Market sell order
/// let request = create_order("BTC_USDT", "sell", "0.001")
///     .order_type("market");
/// let response = client.send(request)?;
/// # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
/// ```
///
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
    CreatePriceOrder::new(
        market,
        trigger_price,
        trigger_rule,
        order_side,
        order_price,
        order_amount,
    )
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

/// Get server current time <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#get-server-current-time)
pub fn get_server_time() -> GetServerTime {
    GetServerTime::new()
}

/// Cancel all open orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#cancel-all-open-orders)
pub fn cancel_all_open_orders() -> CancelAllOpenOrders {
    CancelAllOpenOrders::new()
}

/// Countdown cancel orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#countdown-cancel-orders)
pub fn countdown_cancel_all(timeout: i64) -> CountdownCancelAll {
    CountdownCancelAll::new(timeout)
}

/// Amend multiple orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#batch-modification-of-orders)
pub fn amend_batch_orders(orders: Vec<OrderAmendment>) -> AmendBatchOrders {
    AmendBatchOrders::new(orders)
}

/// Create cross liquidation orders <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#close-position-when-cross-currency-is-disabled)
pub fn create_cross_liquidate_orders(
    orders: Vec<CrossLiquidateOrder>,
) -> CreateCrossLiquidateOrders {
    CreateCrossLiquidateOrders::new(orders)
}

/// Query insurance fund history <br/>
/// [Gate API Documentation](https://www.gate.com/docs/developers/apiv4/#query-spot-insurance-fund-historical-data)
pub fn get_insurance_history(
    business: &str,
    currency: &str,
    from: i64,
    to: i64,
) -> GetInsuranceHistory {
    GetInsuranceHistory::new(business, currency, from, to)
}
