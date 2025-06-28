use serde_json::{json, Value};
use gateio_rs::{
    ureq::GateHttpClient,                // синхронный клиент на базе ureq
};
use gateio_rs::api::spot::{create_order, get_currency_pair, get_currency_pairs, get_ticker};
use gateio_rs::http::Credentials;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    // 1) Create Credentials
    let api_key = "d0e1b2599c9e01315795a87cdc78224a";
    let api_secret = "42f2a62f8f42059930fa96b8d4e0ccab26fcdc3862d2ae20b27ee6c9fe7e389f";
    let credentials = Credentials::new(api_key.to_owned(), api_secret.to_owned());

    // 2) Configure Client
    let client = GateHttpClient::default().credentials(credentials);

    // 3) Send sync request
    // let req = get_ticker()
    //     .currency_pair("BTC_USDT")
    //     .timezone("utc8");

    // let req = get_account();//.currency("USDT");

    // POST запрос с параметрами
    // let req = create_order("LTC_USDT", "buy", "0.04").price("84.2").x_gate_exp_time(1750133608123);
    // let req = create_order("LTC_USDT", "buy", "0.04").price("84.2");
    // let req = create_order("ADA_USDT", "buy", "6").price("0.51");
    // let req = create_order("DUREV_USDT", "buy", "700").price("0.004336");
    // let req = create_order("DUREV_USDT", "sell", "1400").order_type("market");

    // let req = get_currency_pairs();
    let req = get_currency_pair("LTC_USDT");

    // let req = cancel_order("860626791967", "ADA_USDT");
    // let req = amend_order("862872836139", "ADA_USDT").price("0.53");
    // let req = amend_order("863339248610", "DUREV_USDT").price("0.004337").amount("800");
    // let req = cancel_order("862893921486", "DUREV_USDT");

    // let req = get_market_trades("ADA_USDT").last_id("15881638").reverse(true).limit(5);
    // let req = get_batch_user_fee("BTC_USDT,ETH_USDT");
    // let req = get_account_book().book_type("new_order");


    let resp = client.send(req)?;

    // 4) Read response as String
    let body = resp.into_body_str()?;
    // println!("{}", body);

    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);

    Ok(())
}
