use std::{env, fs::File, io::Read, path::Path};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let expr = args.get(1).unwrap();
    let path = args.get(2).unwrap();
    let re = Regex::new(expr.as_str()).unwrap();
    let p = Path::new(path);
    let mut matches : Vec<String> = Vec::new();
    if p.is_dir() {
        parse_dir(&re, &p, &mut matches);
    } else if p.is_file() {
        parse_file(&re, path, &mut matches);
    }
    for m in matches {
        println!("{:?}", m);
    }
}

fn parse_dir(re: &Regex, path: &Path, matches: &mut Vec<String>) {
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

fn parse_file(re: &Regex, path: &str, matches: &mut Vec<String>) {
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
