use std::collections::HashMap;
use reqwest;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Currency {
    base: String,
    date: String,
    rates: HashMap<String, f32>,
}

#[tokio::main]
async fn main() {
    let currency: Currency = serde_json::from_str(reqwest::get("https://api.frankfurter.dev/v1/latest").await.unwrap().text().await.unwrap().as_str()).unwrap();
    let usd = currency.rates.get("USD").unwrap();
    println!("{}", usd);
}
