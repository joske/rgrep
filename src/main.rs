use std::process::exit;

use clap::{arg, command};

use config::Config;

mod search;
mod config;

fn main() {
    let cmd = command!()
        .arg(arg!(-r --recursive).required(false))
        .arg(arg!(-i --ignorecase).required(false))
        .arg(arg!([EXPR] "regex to search for"))
        .arg(arg!([INPUT] "input file"));
    let matches = cmd.get_matches();

    let pstr = matches.value_of("EXPR").unwrap();
    let config = Config {
        expression : String::from(pstr),
        path : String::from(matches.value_of("INPUT").unwrap()),
        recursive : matches.is_present("recursive"),
        ignore_case : matches.is_present("ignorecase"),
    };
    
    let matches = search::search(&config);
    let empty = matches.is_empty();
    for m in matches {
        println!("{:?}", m);
    }
    if empty {
        exit(1);
    }
    exit(0);
}
