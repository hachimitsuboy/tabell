extern crate mecab;
use mecab::Tagger;
use clap::Parser;

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

fn main(){
        // let _arg: AppArg = AppArg::parse();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let mut tagger = Tagger::new("");
        let mut result = tagger.parse_str(input);
        // println!("RESULT: {:?}", result);
        //改行で区切る
        let lineResult: Vec<&str> = result.as_str().split('\n').collect();
         
       
        //タブ文字で区切る。先頭を取り出す
        
        for i in lineResult {
            println!("{:?}", i)

        }
        // println!("{}", _arg.message);

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

