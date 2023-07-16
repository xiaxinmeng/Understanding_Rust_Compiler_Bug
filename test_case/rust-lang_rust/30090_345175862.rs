rust
extern crate regex;
use regex::Regex;

fn main() {
    let input = "aaa\r\nbbb\n\nccc\n";
    let linebreak = Regex::new(r"\r?\n").unwrap();
    let mut prev = 0;
    for m in linebreak.find_iter(input) {
        println!("{}..{} {:?}", prev, m.start(), &input[prev..m.start()]);
        prev = m.end();
    }
    if prev < input.len() {
        println!("{}..{} {:?}", prev, input.len(), &input[prev..]);
    }
}
