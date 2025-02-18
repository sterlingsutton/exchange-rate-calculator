use reqwest;

#[tokio::main]
async fn main() {
    let x = reqwest::get("https://api.frankfurter.dev/v1/latest?base=USD").await.unwrap().text().await.unwrap();
    println!("{}", x);
}
