use clap::Parser;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source .env file path
    #[arg(short, long)]
    input: String,

    /// Template .env file path
    #[arg(short, long)]
    template: String,

    /// Output .env file path
    #[arg(short, long)]
    output: String,
}

fn print_error_and_exit(e: &dyn std::error::Error) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}

fn prompt_file_deletion(path: &String) {
    // prompt
    print!("File exists. Delete? [y/N] (enter any key to abort) ");
    let res_flush = io::stdout().flush();
    if let Err(e) = res_flush {
        print_error_and_exit(&e);
    }

    // get input
    let mut input = String::new();
    let res_input = io::stdin().read_line(&mut input);
    if let Err(e) = res_input {
        print_error_and_exit(&e);
    }

    // parse input
    let input = input.trim().to_lowercase();
    if input == "y" {
        match fs::remove_file(path) {
            Ok(_) => println!("File deleted successfully."),
            Err(e) => print_error_and_exit(&e),
        }
    } else {
        std::process::exit(1);
    }
}

fn prompt_value_replace(key: &String, current: &String, new: &String) -> bool {
    // prompt
    print!(
        "'{}' exists ({} → {}). Replace? [y/N] (enter any key to ignore) ",
        key, current, new
    );
    let res_flush = io::stdout().flush();
    if let Err(e) = res_flush {
        print_error_and_exit(&e);
    }

    // get input
    let mut input = String::new();
    let res_input = io::stdin().read_line(&mut input);
    if let Err(e) = res_input {
        print_error_and_exit(&e);
    }

    // parse input
    let input = input.trim().to_lowercase();
    input == "y"
}

fn main() {
    // open input .env
    let args = Args::parse();
    let env_input = match File::open(&args.input) {
        Ok(f) => f,
        Err(e) => print_error_and_exit(&e),
    };

    // open template .env
    let env_template = match File::open(&args.template) {
        Ok(f) => f,
        Err(e) => print_error_and_exit(&e),
    };

    // delete prompt if file exists
    if Path::new(&args.output).exists() {
        prompt_file_deletion(&args.output);
    }

    // create output
    let env_output = match File::create(&args.output) {
        Ok(f) => f,
        Err(e) => print_error_and_exit(&e),
    };

    // populate source env values
    let mut dict: HashMap<String, String> = HashMap::new();
    for line in io::BufReader::new(env_input).lines().map_while(Result::ok) {
        let res_parse = crate::parser::get_key_val(&line);
        if let Some((key, val)) = res_parse {
            // ask if to replace existing value
            if let Some(current) = dict.get(&key)
                && !prompt_value_replace(&key, current, &val)
            {
                continue;
            }
            dict.insert(key, val);
        }
    }

    // populate source env values
    let mut writer = io::BufWriter::new(env_output);
    for line in io::BufReader::new(env_template).lines().map_while(Result::ok) {
        let res_parse = crate::parser::get_key(&line);

        // transform newline
        let mut line_new = line;
        if let Some(key) = res_parse
            && let Some(val) = dict.get(&key)
        {
            line_new = format!("{}={}", key, val);
        }

        // write new line
        let res_write = writeln!(writer, "{}", line_new);
        if let Err(e) = res_write {
            eprintln!("{}", e);
        }
    }

    println!("File created at '{}'", &args.output)
}
