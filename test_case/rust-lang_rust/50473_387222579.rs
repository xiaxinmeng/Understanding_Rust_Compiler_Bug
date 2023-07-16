rust
extern crate regex;
use regex::Regex;

fn main() {
    let not_subset = Regex::new(r"a?b").unwrap();
    assert!(not_subset.is_match("ab"));
    assert!(not_subset.is_match("b"));
    let subset = Regex::new(r"a?\w").unwrap();
    assert!(subset.is_match("ab"));
    assert!(subset.is_match("aa"));
    assert!(subset.is_match("b"));
    assert!(subset.is_match("a"));
}
