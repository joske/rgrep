use std::{fs::File, io::Read, path::Path};
use regex::Regex;

use crate::config::Config;

pub fn search(config: &Config) -> Vec<String> {
    let mut matches : Vec<String> = Vec::new();
    let re = Regex::new(config.expression.as_str()).unwrap();
    let p = Path::new(config.path.as_str());
    if p.is_dir() {
        parse_dir(config, &re, p, &mut matches);
    } else if p.is_file() {
        parse_file(&re, p, &mut matches);
    }
    matches

}

pub fn parse_dir(config: &Config, re: &Regex, path: &Path, matches: &mut Vec<String>) {
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if entry.path().is_dir() && config.recursive {
                parse_dir(config, re, &entry.path(), matches);
            } else if entry.path().is_file() {
                let n = &entry.path();
                parse_file(re, n, matches);
            }
        }
    }
}

pub fn parse_file(re: &Regex, path: &Path, matches: &mut Vec<String>) {
    println!("Checking {:?}", path);
    let file = File::open(path);
    if file.is_ok() {
        let mut file = file.unwrap();
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            for line in contents.lines() {
                if re.is_match(line) {
                    let m = format!("{} : {}", path.to_str().unwrap(), line);
                    matches.push(m);
                }
            }
        }
    }
}