rust
use std::{collections::HashMap, thread, time::Duration};

fn func() {
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();

    for i in 0..100_000 {
        let key = format!("{}", i);
        map.insert(key, vec![0; 50_000]);
    }
}

fn main() {
    func();
    println!("LOOP");
    loop {
        thread::sleep(Duration::from_secs(5));
    }
}
