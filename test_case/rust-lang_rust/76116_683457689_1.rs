rust
use serde_json::json;
#[test]
fn test_json() {
    let arg = json!(1);
    println!("Hello, world! {}",arg);
}


fn main() {
    println!("Hello, world!");
}
