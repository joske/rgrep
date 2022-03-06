use std::{env, path::Path};
use clap::{arg, command, Command};

use config::Config;
use regex::Regex;

mod search;
mod config;

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let expr = args.get(1).unwrap();
    let path = args.get(2).unwrap();
    Config { expression: expr, path: *path, recursive: false }
}

fn main() {
    let mut cmd = clap::command!()
        // Add the version arguments
        .arg(arg!(--"set-ver" <VER> "set version manually").required(false))
        .arg(arg!(--major         "auto inc major"))
        .arg(arg!(--minor         "auto inc minor"))
        .arg(arg!(--patch         "auto inc patch"))
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(arg!([INPUT_FILE] "some regular input"))
        .arg(arg!(--"spec-in" <SPEC_IN> "some special input argument").required(false))
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(arg!(config: -c <CONFIG>).required(false));
    let matches = cmd.get_matches_mut();

    let config = parse_args();
    let re = Regex::new(config.expression.as_str()).unwrap();
    let p = Path::new(config.path.as_str());
    let mut matches : Vec<String> = Vec::new();
    if p.is_dir() {
        search::parse_dir(&re, &p, &mut matches);
    } else if p.is_file() {
        search::parse_file(&re, config.path.as_str(), &mut matches);
    }
    for m in matches {
        println!("{:?}", m);
    }
}
