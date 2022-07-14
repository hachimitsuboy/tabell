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





use reqwest::Client;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[tokio::main]

async fn main() -> Result<()>{
        // let _arg: AppArg = AppArg::parse();
        let args: Vec<String> = env::args().collect();
        let input = &args[1];
    
        let url = "https://api.codic.jp/v1/engine/translate.json?text=".to_string() + input + "&casing=camel";


        let client = Client::new();
        let res = client.get(url).header("Authorization", "Bearer n62BwX8uDpShpvXkBlnJNNsqrApyd9r0QI").header("Host", "api.codic.jp").send().await?;

        let body = res.text().await?;

    
        let split_body: Vec<&str> = body.split("\"").collect();
        println!("{:?}", split_body[9]);

        

        let json: serde_json::Value = serde_json::from_str(&body)?;

        let obj = json.as_object().unwrap();

        println!("{:?}", obj);


        for (key,value) in obj.iter() {
            println!("{:?}\t{:?}",key,value);
            }
    
        eprintln!("*** 終了 ***");
        Ok(())
    

            
    
        // println!("{}", type_of(&res));
        // println!("Response: {:?}", res);

        // let mut tagger = Tagger::new("");

        // println!("{}", input);
        // let mut result = tagger.parse_str(input);
        // // println!("RESULT: {:?}", result);
        // //改行で区切る
        // let lineResult: Vec<&str> = result.as_str().split('\n').collect();
         
       
        // //タブ文字で区切る。先頭を取り出す
        
        // for i in lineResult {
        //     println!("{:?}", i)

        // }
        // // println!("{}", _arg.message);
        




}


fn request(input: &str){
    let url = "https://api.codic.jp/v1/engine/translate.json?text=".to_string() + input + "&casing=lower+underscore";


    let client = reqwest::blocking::Client::new();
    let res = client.get(url).header("Authorization", "Bearer n62BwX8uDpShpvXkBlnJNNsqrApyd9r0QI").header("Host", "api.codic.jp").send().unwrap().text();

    println!("{}", type_of(&res));
    println!("////////////////");
    println!("Response: {:?}", res);

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
}

