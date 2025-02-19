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

struct Exchange {
    base: String,
    target: String,
    amount: f32,
}

#[tokio::main]
async fn main() {
    let currencies: HashMap<String, String> = serde_json::from_str(reqwest::get("https://api.frankfurter.dev/v1/currencies").await.unwrap().text().await.unwrap().as_str()).unwrap();
    
    loop {
        list_currencies(&currencies);

    }

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

fn get_exchange_info() -> Exchange {
    // Set default exchange values
    let mut exchange = Exchange {
        base: "USD".to_owned(),
        target: "EUR".to_owned(),
        amount: 10.0,
    };

    // incomplete
}
