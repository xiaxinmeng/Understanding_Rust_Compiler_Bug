 rust
struct Mine<'a> { s: &'a str }

impl <'a>Iterator for Mine<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&str> { Some("h") }
}

fn main() {}
