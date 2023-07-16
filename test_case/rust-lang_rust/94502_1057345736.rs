`rust
use std::collections::HashMap;

pub fn f(input: &str) {
    input
        .lines()
        .map(|s| -> (u32) { (1) })
        .fold(HashMap::new(), |mut map, (x)| {
            let prev = map.get(&(0, x)).unwrap_or(&0);
            map.insert((0, x), prev + 1);
            map
        });
}

pub fn main() {
    let _ = f("");
}
