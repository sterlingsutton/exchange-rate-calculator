// Copyright (c) 2025 Sterling Sutton

use std::collections::HashMap;
use reqwest;
use serde_json::{self, Value};
use text_io::Error;

struct Exchange {
    base: String,
    target: String,
    amount: f32,
}

impl Exchange {
    async fn calculate_target_amount(&self) -> f32 {
        let url = format!("https://api.frankfurter.dev/v1/latest?base={}", self.base);

        let data: Value = serde_json::from_str(reqwest::get(url).await.unwrap().text().await.unwrap().as_str()).unwrap();

        let rate = data["rates"][&self.target].as_f64().unwrap() as f32;

        rate * self.amount
    }
}

#[tokio::main]
async fn main() {
    let currencies: HashMap<String, String> = serde_json::from_str(reqwest::get("https://api.frankfurter.dev/v1/currencies").await.unwrap().text().await.unwrap().as_str()).unwrap();
    list_currencies(&currencies);
    let exchange = get_exchange_info(&currencies);
    let target_amount = exchange.calculate_target_amount().await;
    println!("Target Amount ({}) : {:.2}", exchange.target, target_amount);

}

fn list_currencies(currencies: &HashMap<String, String>) {
    println!("---- AVAILABLE CURRENCIES ----");
    for (symbol, name) in currencies {
        let name = name.to_uppercase();
        println!("{} : {}", symbol, name);
    }
    println!();
}

fn get_exchange_info(currencies: &HashMap<String, String>) -> Exchange {
    // Set default exchange values
    println!("---- EXCHANGE INFO ----");
    let mut exchange = Exchange {
        base: "USD".to_owned(),
        target: "EUR".to_owned(),
        amount: 10.0,
    };
    // get base
    loop {
        print!("Base Currency (default USD) : ");
        let base: String = text_io::read!();
        // change base and break from loop if base is valid
        if currencies.contains_key(&base) {
            exchange.base = base;
            break;
        }
    }
    // get amount
    loop {
        print!("Amount ({}) : ", exchange.base);
        let amount: Result<f32, Error> = text_io::try_read!();

        match amount.is_ok() {
            true => {
                let amount = amount.unwrap();
                if amount > 0.0 {
                    exchange.amount = amount;
                    break;
                }
            }
            false => {
                // do nothing
            }
        }

        
    }
    // get target
    loop {
        print!("Target Currency (default EUR) : ");
        let target: String = text_io::read!();
        // same thing as with the base
        if currencies.contains_key(&target) && !target.eq(&exchange.base) {
            exchange.target = target;
            break;
        }
    }

    exchange
}
