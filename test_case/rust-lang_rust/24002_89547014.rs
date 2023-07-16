 rust
use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let map = (|| {
        let mut map = HashMap::new();
        map.insert("imo", "in my opinion");
        map
    })();

    for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
        println!("{}", line.split(' ')
                 .map(|seg| if map.contains_key(seg) {
                     map[seg.as_slice()].to_string()
                 } else {
                     seg.to_string()
                 }).collect::<Vec<String>>().connect(" "));
    }
}
