use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::config::Config;

pub fn search(config: &Config) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    let mut expr  = String::new();
    if config.ignore_case {
        expr.push_str("(?i)");
    }
    expr.push_str(config.expression.as_str());
    let re = Regex::new(&expr).unwrap();
    let p = Path::new(config.path.as_str());
    if p.is_dir() {
        parse_dir(config, &re, p, &mut matches);
    } else if p.is_file() {
        parse_file(config, &re, p, &mut matches);
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
                parse_file(config, re, n, matches);
            }
        }
    }
}

pub fn parse_file(config: &Config, re: &Regex, path: &Path, matches: &mut Vec<String>) {
    let file = File::open(path);
    if file.is_ok() {
        let file = file.unwrap();
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if config.fixed_strings {
                    if config.invert ^ line.contains(config.expression.as_str()) {
                        add_match(path, index, line, matches);
                    }
                } else {
                    if config.invert ^ re.is_match(line.as_str()) {
                        add_match(path, index, line, matches);
                    }
                }
            }
        }
    }
}

fn add_match(path: &Path, index: usize, line: String, matches: &mut Vec<String>) {
    let m = format!("{}:{} : {}", path.to_str().unwrap(), index, line);
    matches.push(m);
}
