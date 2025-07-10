use gateio_rs::{api::spot, http::Credentials, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    // Load credentials from environment
    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // Create order amendments
    let amendments = vec![
        spot::OrderAmendment::new("order_id_1", "BTC_USDT")
            .price("45000")
            .amount("0.01"),
        spot::OrderAmendment::new("order_id_2", "ETH_USDT")
            .price("3000")
            .amend_text("Updated price"),
    ];

    // Create request
    let req = spot::amend_batch_orders(amendments);

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);

    Ok(())
}
