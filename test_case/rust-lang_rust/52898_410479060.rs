rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap< (), &u32 > = HashMap::new();
    map.insert( (), &123 );
    for (_, v) in map.drain() {
        println!( "{}", v );
    }
}
