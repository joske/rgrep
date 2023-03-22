#[allow(clippy::struct_excessive_bools)]
pub struct Config {
    pub expression: String,
    pub path: String,
    // options
    pub recursive: bool,
    pub ignore_case: bool,
    pub fixed_strings: bool,
    pub invert: bool,
}

impl Config {
    #[allow(dead_code)] // constructor is only used in unit tests
    pub fn new(expr: String, path: String) -> Self {
        Config {
            expression: (expr),
            path: (path),
            recursive: (false),
            ignore_case: (false),
            fixed_strings: (false),
            invert: (false),
        }
    }
}
