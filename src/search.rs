use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::config::Config;

pub fn search(config: &Config) -> Result<Vec<String>, String> {
    let mut matches: Vec<String> = Vec::new();

    let mut expr = String::with_capacity(config.expression.len() + 10);
    if config.ignore_case {
        expr.push_str("(?i)");
    }
    expr.push_str(config.expression.as_str());
    let re = Regex::new(&expr).map_err(|e| e.to_string())?;
    let p = Path::new(config.path.as_str());
    if p.is_dir() {
        parse_dir(config, &re, p, &mut matches)?;
    } else if p.is_file() {
        parse_file(config, &re, p, &mut matches)?;
    } else {
        return Err(format!("path not found: {}", config.path));
    }
    Ok(matches)
}

fn parse_dir(
    config: &Config,
    re: &Regex,
    path: &Path,
    matches: &mut Vec<String>,
) -> Result<(), String> {
    for entry in (path.read_dir().map_err(|e| e.to_string())?).flatten() {
        if entry.path().is_dir() && config.recursive {
            parse_dir(config, re, &entry.path(), matches)?;
        } else if entry.path().is_file() {
            let n = &entry.path();
            parse_file(config, re, n, matches)?;
        }
    }
    Ok(())
}

fn parse_file(
    config: &Config,
    re: &Regex,
    path: &Path,
    matches: &mut Vec<String>,
) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            match_line(config, &line, path, index, matches, re);
        }
    }
    Ok(())
}

fn match_line(
    config: &Config,
    line: &str,
    path: &Path,
    index: usize,
    matches: &mut Vec<String>,
    re: &Regex,
) {
    let is_match = if config.fixed_strings {
        line.contains(config.expression.as_str())
    } else {
        re.is_match(line)
    };
    match (is_match, config.invert) {
        (true, false) | (false, true) => add_match(path, index, line, matches),
        _ => (),
    }
}

fn add_match(path: &Path, index: usize, line: &str, matches: &mut Vec<String>) {
    let m = format!("{}:{index} : {line}", path.display(),);
    matches.push(m);
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::*;

    #[test]
    fn test_fixed_strings() {
        let mut config = Config::new(String::from("needle"), String::from("/tmp/foo"));
        config.fixed_strings = true;
        let mut matches: Vec<String> = Vec::new();
        let re = Regex::new(config.expression.as_str()).unwrap();
        let p = Path::new(config.path.as_str());
        match_line(
            &config,
            "this line should match needle",
            p,
            1,
            &mut matches,
            &re,
        );
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_fixed_strings_invert() {
        let mut config = Config::new(String::from("needle"), String::from("/tmp/foo"));
        config.fixed_strings = true;
        config.invert = true;
        let mut matches: Vec<String> = Vec::new();
        let re = Regex::new(config.expression.as_str()).unwrap();
        let p = Path::new(config.path.as_str());
        match_line(
            &config,
            "this line should match needle",
            p,
            1,
            &mut matches,
            &re,
        );
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_fixed_strings_no_match() {
        let mut config = Config::new(String::from("blahblah"), String::from("/tmp/foo"));
        config.fixed_strings = true;
        let mut matches: Vec<String> = Vec::new();
        let re = Regex::new(config.expression.as_str()).unwrap();
        let p = Path::new(config.path.as_str());
        match_line(
            &config,
            "this line should match needle\nthis one shouldn't",
            p,
            1,
            &mut matches,
            &re,
        );
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_fixed_strings_no_match_invert() {
        let mut config = Config::new(String::from("blahblah"), String::from("/tmp/foo"));
        config.fixed_strings = true;
        config.invert = true;
        let mut matches: Vec<String> = Vec::new();
        let re = Regex::new(config.expression.as_str()).unwrap();
        let p = Path::new(config.path.as_str());
        match_line(
            &config,
            "this line should match needle\nthis one shouldn't",
            p,
            1,
            &mut matches,
            &re,
        );
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_regex() {
        let config = Config::new(String::from("n[e]*dle"), String::from("/tmp/foo"));
        let mut matches: Vec<String> = Vec::new();
        let re = Regex::new(config.expression.as_str()).unwrap();
        let p = Path::new(config.path.as_str());
        match_line(
            &config,
            "this line should match needle\nthis one shouldn't",
            p,
            1,
            &mut matches,
            &re,
        );
        assert_eq!(matches.len(), 1);
    }
}
