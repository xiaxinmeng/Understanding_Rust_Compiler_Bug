rust
use std::{collections::HashMap, thread, time::Duration};

fn func() {
    let mut map: HashMap<i32, Vec<u8>> = HashMap::new();

    for num in 0..100_000 {
        
        let key = format!("{}_test", num);

        // This line causes HashMap<i32, Vec<u8>> not to release memory.. Why ?
        let _key: &'static str = Box::leak(key.into_boxed_str());

        map.insert(num, vec![0; 50_000]);
    }
}

fn main() {
    func();
    println!("LOOP");
    loop {
        thread::sleep(Duration::from_secs(5));
    }
}
