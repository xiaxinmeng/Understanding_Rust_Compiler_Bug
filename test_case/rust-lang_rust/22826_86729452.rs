 rust
use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("a".to_string(), 2);
    println!("{}", m["a"]);
}
