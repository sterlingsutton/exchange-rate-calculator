// Copyright (c) 2025 Sterling Sutton

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
    let currency_options: HashMap<String, String> = serde_json::from_str(reqwest::get("https://api.frankfurter.dev/v1/currencies").await.unwrap().text().await.unwrap().as_str()).unwrap();

    list_currencies(&currency_options);

    //let currency: Currency = serde_json::from_str(reqwest::get("https://api.frankfurter.dev/v1/latest").await.unwrap().text().await.unwrap().as_str()).unwrap();
}

fn list_currencies(currencies: &HashMap<String, String>) {
    println!("---- AVAILABLE CURRENCIES ----");
    for (symbol, name) in currencies {
        let name = name.to_uppercase();
        println!("{} : {}", symbol, name);
    }
    println!("---- END ----");
}
