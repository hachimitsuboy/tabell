extern crate serde;
extern crate serde_json;
extern crate reqwest;
use clap::Parser;
use std::env;


#[derive(Parser)]
#[clap(
    name = "tabell",
    author = "Akinobu Nagae",
    version = "0.1.0",
    about = "Rust CLI tabell"
)]
    
struct AppArg {
    /// generate variable names with snake case
    #[clap(short = 's', long = "snake")]
    snake: Option<String>,

    /// generate variable 
    /// names with kebab case
    #[clap(short = 'k', long = "kabab")]
    kabab: Option<String>,

    //位置引数
    message: String, 

}


#[tokio::main]
async fn main() {

        let args: Vec<String> = env::args().collect();
        let input = &args[1];
        let res = request(input).await;
        println!("{}", res.unwrap());

}

use reqwest::Client;
async fn request(input: &str) -> Result<String, Box<dyn std::error::Error>>{
 
    let url = "https://api.codic.jp/v1/engine/translate.json?text=".to_string() + input + "&casing=camel";

    let client = Client::new();
    let res = client.get(url).header("Authorization", "Bearer n62BwX8uDpShpvXkBlnJNNsqrApyd9r0QI").header("Host", "api.codic.jp").send().await?;

    let body = res.text().await?;
    
    let split_body: Vec<&str> = body.split("\"").collect();
    let result = split_body[9].to_string();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn response() {
        let r1 = request("大きな時計").await.unwrap();
        assert_eq!("largeClock", r1);
    }

}

