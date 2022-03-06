use clap::{arg, command};

use config::Config;

mod search;
mod config;

fn main() {
    let mut cmd = command!()
        // Add the version arguments
        .arg(arg!(-r --recursive).required(false))
        .arg(arg!(-i --ignorecase).required(false))
        .arg(arg!([EXPR] "regex to search for"))
        .arg(arg!([INPUT] "input file"));
    let matches = cmd.get_matches_mut();

    let pstr = matches.value_of("EXPR").unwrap();
    let config = Config {
        expression : String::from(pstr),
        path : String::from(matches.value_of("INPUT").unwrap()),
        recursive : matches.is_present("recursive"),
        ignore_case : matches.is_present("ignorecase"),
    };
    
    let matches = search::search(&config);
    for m in matches {
        println!("{:?}", m);
    }
}
