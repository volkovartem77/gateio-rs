use serde_json::Value;
use gateio_rs::{
    ureq::GateHttpClient,
    api::spot,
    http::Credentials,
};

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();
    
    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);
    
    let client = GateHttpClient::default().credentials(credentials.clone());
    
    // Create a price-triggered order
    // This example creates a buy order that will trigger when BTC price goes above $50,000
    // When triggered, it will place a limit buy order for 0.001 BTC at $49,900
    let req = spot::create_price_order(
        "BTC_USDT",    // market (currency pair)
        "120000",       // trigger_price
        ">=",          // trigger_rule (trigger when price >= $50,000)
        "buy",         // order_side
        "49900",       // order_price (limit price when triggered)
        "0.001"        // order_amount
    )
    .account("normal")           // use normal spot account
    .time_in_force("gtc")        // good till cancelled
    .trigger_expiration(86400);  // expire after 24 hours if not triggered
    
    println!("Sending price-triggered order request...");
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    
    println!("Response: {:#}", resp_obj);
    
    Ok(())
}