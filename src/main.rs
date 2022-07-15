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
        // let _arg: AppArg = AppArg::parse();
        let args: Vec<String> = env::args().collect();
        let input = &args[1];
        println!("input: {}", input);
        let res = request(input).await;
        println!("{}",type_of(&res));

}

use reqwest::Client;
type Result<String> = std::result::Result<String, Box<dyn std::error::Error>>;

async fn request(input: &str) -> Result<()>{
 
    let url = "https://api.codic.jp/v1/engine/translate.json?text=".to_string() + input + "&casing=camel";

    let client = Client::new();
    let res = client.get(url).header("Authorization", "Bearer n62BwX8uDpShpvXkBlnJNNsqrApyd9r0QI").header("Host", "api.codic.jp").send().await?;

    let body = res.text().await?;
    
    let split_body: Vec<&str> = body.split("\"").collect();
    println!("{:?}", split_body[9]);   
    

    Ok(())

}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn hello(name: Option<String>) -> String {
    return format!("Hello, {}", if let Some(n) = name {
        n
    } else {
        "World".to_string()
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("Hello, World", hello(None));
        assert_eq!("Hello, Nagae", hello(Some("Nagae".to_string())));
    }

    fn test_input() {
        let r1 = request("大きな時計").await.unwrap();
        assert_eq!("largeClock", r1);
    }

}

