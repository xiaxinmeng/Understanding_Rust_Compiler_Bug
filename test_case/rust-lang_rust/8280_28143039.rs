 rust
use std::hashmap;

fn main() {
    let mut map = hashmap::HashMap::new();

    let str = "Hello, world!";

    for &char in str.to_ascii().iter() {
        map.insert_or_update_with(char, 1, |_, v| *v += 1);
    }

    println!("{:?}", map);
}
