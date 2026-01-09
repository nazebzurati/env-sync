use crate::print;
use std::fs::{self};
use std::io::{self, Write};

pub fn file_deletion(path: &String) {
    // prompt
    print!("File exists. Delete? [y/N] (enter any key to abort) ");
    let res_flush = io::stdout().flush();
    if let Err(e) = res_flush {
        print::error_and_exit(&e);
    }

    // get input
    let mut input = String::new();
    let res_input = io::stdin().read_line(&mut input);
    if let Err(e) = res_input {
        print::error_and_exit(&e);
    }

    // parse input
    let input = input.trim().to_lowercase();
    if input == "y" {
        match fs::remove_file(path) {
            Ok(_) => println!("File deleted successfully."),
            Err(e) => print::error_and_exit(&e),
        }
    } else {
        std::process::exit(1);
    }
}

pub fn value_replace(key: &String, current: &String, new: &String) -> bool {
    // prompt
    print!(
        "'{}' exists ({} → {}). Replace? [y/N] (enter any key to ignore) ",
        key, current, new
    );
    let res_flush = io::stdout().flush();
    if let Err(e) = res_flush {
        print::error_and_exit(&e);
    }

    // get input
    let mut input = String::new();
    let res_input = io::stdin().read_line(&mut input);
    if let Err(e) = res_input {
        print::error_and_exit(&e);
    }

    // parse input
    let input = input.trim().to_lowercase();
    input == "y"
}
