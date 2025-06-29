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
    
    // You need to replace this with an actual price order ID
    // You can get order IDs from the get_price_orders example
    let order_id = "1037810722"; // Replace with actual order ID
    
    println!("This example will cancel price-triggered order with ID: {}", order_id);
    
    let req = spot::cancel_price_order(order_id);
    
    // Uncomment the lines below to actually send the request
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);
    
    Ok(())
}