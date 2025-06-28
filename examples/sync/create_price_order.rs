use serde_json::Value;
use gateio_rs::{
    ureq::GateHttpClient,
};
use gateio_rs::api::spot::create_price_order;
use gateio_rs::http::Credentials;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();
    
    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);
    
    let client = GateHttpClient::default().credentials(credentials.clone());
    
    // Create a price-triggered order
    // This will create a buy order for 0.01 BTC when the price drops to $60,000
    let req = create_price_order("BTC_USDT", "60000", "buy", "0.01")
        .credentials(credentials);
    
    println!("Creating price-triggered order...");
    println!("Currency Pair: BTC_USDT");
    println!("Trigger Price: $60,000");
    println!("Side: buy");
    println!("Amount: 0.01 BTC");
    println!("Execution Price: $59,500");
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("Price-triggered order response: {:?}", resp_obj);
    
    Ok(())
}