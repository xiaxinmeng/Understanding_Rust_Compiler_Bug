 rust
struct str {
    x: i8
}

fn main() {
    let welp = str { x: 42 };
    let welp: &str = &welp; //~ ERROR mismatched types: expected `&str` but found `&str` (expected str but found struct str)
}
