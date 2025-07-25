use gateio_rs::{
    api::spot::{
        create_batch_orders, create_order, get_account, get_account_book, get_batch_user_fee,
        get_currency_pair, get_currency_pairs, get_ticker, Order,
    },
    http::Credentials,
    ureq::GateHttpClient,
};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Create Credentials
    // TODO: Replace with your actual API credentials or use environment variables
    let api_key = "YOUR_GATE_API_KEY";
    let api_secret = "YOUR_GATE_API_SECRET";
    let credentials = Credentials::new(api_key.to_owned(), api_secret.to_owned());

    // 2) Configure Client
    let client = GateHttpClient::default().credentials(credentials);

    // 3) Send sync request examples
    
    // Example 1: Get ticker
    println!("Getting ticker for BTC_USDT...");
    let req = get_ticker()
        .currency_pair("BTC_USDT")
        .timezone("utc8");
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let ticker_data: Value = serde_json::from_str(&body)?;
    println!("Ticker: {}\n", serde_json::to_string_pretty(&ticker_data)?);

    // Example 2: Get account information
    println!("Getting account information...");
    let req = get_account();
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let account_data: Value = serde_json::from_str(&body)?;
    println!("Account: {}\n", serde_json::to_string_pretty(&account_data)?);

    // Example 3: Get currency pairs
    println!("Getting currency pairs...");
    let req = get_currency_pairs();
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let pairs_data: Value = serde_json::from_str(&body)?;
    println!("Found {} currency pairs\n", pairs_data.as_array().map(|a| a.len()).unwrap_or(0));

    // Example 4: Get specific currency pair
    println!("Getting LTC_USDT currency pair info...");
    let req = get_currency_pair("LTC_USDT");
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let pair_data: Value = serde_json::from_str(&body)?;
    println!("LTC_USDT info: {}\n", serde_json::to_string_pretty(&pair_data)?);

    // Example 5: Create order
    println!("Creating order...");
    let req = create_order("LTC_USDT", "buy", "0.04").price("84.2");

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let order_data: Value = serde_json::from_str(&body)?;
    println!("Order created: {}\n", serde_json::to_string_pretty(&order_data)?);

    // Example 6: Batch user fee
    println!("Getting batch user fee...");
    let req = get_batch_user_fee("BTC_USDT,ETH_USDT");
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let fee_data: Value = serde_json::from_str(&body)?;
    println!("Fees: {}\n", serde_json::to_string_pretty(&fee_data)?);

    // Example 7: Account book
    println!("Getting account book...");
    let req = get_account_book().book_type("new_order");
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let book_data: Value = serde_json::from_str(&body)?;
    println!("Account book: {}\n", serde_json::to_string_pretty(&book_data)?);

    // Example 8: Batch orders
    println!("Creating batch orders...");
    let order1 = Order::new("BTC_USDT", "buy", "0.001")
        .text("t-abc123")
        .order_type("limit")
        .account("unified")
        .price("65000")
        .time_in_force("gtc")
        .iceberg("0");

    let order2 = Order::new("ETH_USDT", "buy", "0.01")
        .text("t-def456")
        .order_type("limit")
        .account("unified")
        .price("3000")
        .time_in_force("gtc")
        .iceberg("0");

    let orders = vec![order1, order2];
    let req = create_batch_orders(orders);

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let batch_data: Value = serde_json::from_str(&body)?;
    println!("Batch orders: {}\n", serde_json::to_string_pretty(&batch_data)?);

    println!("All sync examples completed successfully!");
    Ok(())
}