use serde_json::Value;
use gateio_rs::{
    ureq::GateHttpClient,
    api::spot::{self, CancelOrderRequest},
    http::Credentials,
};

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();
    
    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);
    
    let client = GateHttpClient::default().credentials(credentials.clone());
    
    // Create a list of orders to cancel - replace with actual order IDs
    let orders_to_cancel = vec![
        CancelOrderRequest {
            id: "123456789".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        },
        CancelOrderRequest {
            id: "987654321".to_string(),
            currency_pair: "ETH_USDT".to_string(),
        },
    ];
    
    let req = spot::cancel_batch_orders(orders_to_cancel);
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);
    
    Ok(())
}