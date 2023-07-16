rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), 10);
    
    let value = {
        let key = "foo".to_string();
        map.get(&key)
    };
    
    println!("{}", value.unwrap());
}
