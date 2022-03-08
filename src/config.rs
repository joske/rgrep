pub struct Config {
    pub expression: String,
    pub path: String,
    // options
    pub recursive: bool,
    pub ignore_case: bool,
    pub fixed_strings: bool,
    pub invert: bool,
}