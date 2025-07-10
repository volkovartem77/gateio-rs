use gateio_rs::{
    api::spot::{self, Order},
    http::Credentials,
    ureq::GateHttpClient,
};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // Create multiple orders
    let orders = vec![
        Order::new("BTC_USDT", "buy", "0.001")
            .order_type("limit")
            .price("30000")
            .account("spot"),
        Order::new("ETH_USDT", "buy", "0.01")
            .order_type("limit")
            .price("2000")
            .account("spot"),
    ];

    let req = spot::create_batch_orders(orders);

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);

    Ok(())
}
