use gateio_rs::{api::spot, http::Credentials, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // Create a limit order
    // let req = spot::create_order("BTC_USDT", "buy", "0.001")
    //     .order_type("limit")
    //     .price("30000")
    //     .time_in_force("gtc")
    //     .account("spot");

    // Create a market order
    let req = spot::create_order("DUREV_USDT", "sell", "0")
        .order_type("market")
        .time_in_force("ioc");
    // .account("spot");

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);

    Ok(())
}
