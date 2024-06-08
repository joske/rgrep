use std::process::exit;

use clap::{arg, command};

use config::Config;

mod config;
mod search;

fn main() {
    let cmd = command!()
        .arg(arg!(-r - -recursive).required(false))
        .arg(arg!(-i - -ignorecase).required(false))
        .arg(arg!(-v - -invert).required(false))
        .arg(arg!(-F - -fixedstrings).required(false))
        .arg(arg!([EXPR] "regex to search for"))
        .arg(arg!([INPUT] "input file"));
    let matches = cmd.get_matches();

    let input: Option<&String> = matches.get_one("INPUT");
    let expr: Option<&String> = matches.get_one("EXPR");
    if let (Some(pstr), Some(input)) = (expr, input) {
        let config = Config {
            expression: String::from(pstr),
            path: String::from(input),
            recursive: matches.contains_id("recursive"),
            ignore_case: matches.contains_id("ignorecase"),
            fixed_strings: matches.contains_id("fixedstrings"),
            invert: matches.contains_id("invert"),
        };

        let matches = search::search(&config);
        match matches {
            Ok(matches) => {
                let empty = matches.is_empty();
                for m in matches {
                    println!("{m:?}");
                }
                if empty {
                    exit(1);
                }
                exit(0);
            }
            Err(e) => {
                println!("Error: {e}");
                exit(-1);
            }
        }
    } else {
        println!("Usage: rgrep <expr> <input>");
        exit(-2);
    }
}
