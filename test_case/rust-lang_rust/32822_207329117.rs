 rust
fn main() {
    let v = vec!["abc".to_string(), "def".to_string()];

    let s: String = "def".to_string();
    println!("{:?}", v.binary_search(&s));
}
