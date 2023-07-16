rust
use regex;

fn main() {
    regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
}
