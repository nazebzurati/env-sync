use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
mod parser;
mod print;
mod prompt;

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

struct EnvInfo {
    value: String,
    is_used: bool,
}

fn main() {
    // open input .env
    let args = Args::parse();
    let env_input = match File::open(&args.input) {
        Ok(f) => f,
        Err(e) => print::error_and_exit(&e),
    };

    // open template .env
    let env_template = match File::open(&args.template) {
        Ok(f) => f,
        Err(e) => print::error_and_exit(&e),
    };

    // delete prompt if file exists
    if Path::new(&args.output).exists() {
        prompt::file_deletion(&args.output);
    }

    // create output
    let env_output = match File::create(&args.output) {
        Ok(f) => f,
        Err(e) => print::error_and_exit(&e),
    };

    // populate source env values
    let mut dict: HashMap<String, EnvInfo> = HashMap::new();
    for line in io::BufReader::new(env_input).lines().map_while(Result::ok) {
        let res_parse = crate::parser::get_key_val(&line);
        if let Some((key, val)) = res_parse {
            // ask if to replace existing value
            if let Some(current) = dict.get(&key)
                && !prompt::value_replace(&key, &current.value, &val)
            {
                continue;
            }
            dict.insert(
                key,
                EnvInfo {
                    value: val,
                    is_used: false,
                },
            );
        }
    }

    // populate source env values
    let mut writer = io::BufWriter::new(env_output);
    for line in io::BufReader::new(env_template).lines().map_while(Result::ok) {
        let res_parse = crate::parser::get_key(&line);

        // transform newline
        let mut line_new = line;
        if let Some(key) = res_parse
            && let Some(item) = dict.get(&key)
        {
            line_new = format!("{}={}", key, item.value);

            // update if key used
            dict.entry(key).and_modify(|item| {
                item.is_used = true;
            });
        }

        // write new line
        let res_write = writeln!(writer, "{}", line_new);
        if let Err(e) = res_write {
            eprintln!("{}", e);
        }
    }

    // filter used variable and sort by key alphabetically
    let mut items: Vec<_> = dict.iter().collect();
    items.sort_by(|a, b| a.0.cmp(b.0));
    items.retain(|(_k, v)| !v.is_used);

    // populate unused variable
    let res_unused_title = writeln!(writer, "\n# Unused variables");
    if let Err(e) = res_unused_title {
        eprintln!("{}", e);
    }
    for (key, item) in items.iter() {
        let res_write = writeln!(writer, "# {}={}", key, item.value);
        if let Err(e) = res_write {
            eprintln!("{}", e);
        }
    }

    println!("File created at '{}'", &args.output)
}
