use std::{env, fs::File, io::Read, path::Path};
use regex::Regex;

pub fn parse_dir(re: &Regex, path: &Path, matches: &mut Vec<String>) {
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                parse_dir(re, &entry.path(), matches);
            } else if entry.path().is_file() {
                let n = &entry.path();
                parse_file(re, n.to_str().unwrap(), matches);
            }
        }
    }
}

pub fn parse_file(re: &Regex, path: &str, matches: &mut Vec<String>) {
    println!("{:?}", path);
    let file = File::open(path);
    if file.is_ok() {
        let mut file = file.unwrap();
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            for line in contents.lines() {
                if re.is_match(line) {
                    let m = format!("{} : {}", path, line);
                    matches.push(m);
                }
            }
        }
    }
}