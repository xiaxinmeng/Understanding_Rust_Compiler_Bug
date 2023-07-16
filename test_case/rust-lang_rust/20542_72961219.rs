 rust
use std::str::FromStr;

fn foo<T: std::str::FromStr> (s: &str) {
    let t: Result<T,FromStr::Err> = FromStr::from_str(s);
}

fn main() { }
