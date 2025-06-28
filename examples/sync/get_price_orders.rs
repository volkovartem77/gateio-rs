use serde_json::Value;
use gateio_rs::{
    ureq::GateHttpClient,
};
use gateio_rs::api::spot::get_price_orders;
use gateio_rs::http::Credentials;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();
    
    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);
    
    let client = GateHttpClient::default().credentials(credentials.clone());
    
    // Test 1: Get all price-triggered orders
    let req = get_price_orders()
        .limit(10)
        .credentials(credentials.clone());
    
    println!("Fetching all price-triggered orders...");
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("All price orders: {:?}", resp_obj);
    
    // Test 2: Get price orders for specific currency pair (simplified)
    let req = get_price_orders()
        .currency_pair("BTC_USDT")
        .credentials(credentials.clone());
    
    println!("\nFetching BTC_USDT price-triggered orders...");
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("BTC_USDT price orders: {:?}", resp_obj);
    
    Ok(())
}