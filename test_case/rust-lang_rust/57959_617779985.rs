rust
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    
    let cursor = io::Cursor::new(b"lorem\nipsum\r\ndolor");
    let reader = cursor.lines();
    let mut map = HashMap::new();
    
    for line in reader {
        let mut split = line.unwrap().split(":");

        let name = split.next().unwrap().trim();
        let value = split.next().unwrap().trim();
        
        map.insert(name, value);
    }
}
