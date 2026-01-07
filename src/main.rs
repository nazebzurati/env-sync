use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source .env file path
    #[arg(short, long)]
    source: String,

    /// Template .env file to be filled
    #[arg(short, long)]
    template: String,
}

fn main() {
    // read arguments
    let args = Args::parse();
    let file = match File::open(&args.source) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // populate source env values
    let mut dict: HashMap<String, String> = HashMap::new();
    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        let res = crate::parser::parse_env_line(&line);
        if let Some((key, val)) = res {
            dict.insert(key, val);
        }
    }
}
