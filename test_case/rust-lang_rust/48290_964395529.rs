rust
use std::time::{SystemTime, UNIX_EPOCH};

fn get_name() -> impl ToString {
    let time = SystemTime::now();
    let timestamp = time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    if  timestamp % 2 == 0 {
        variant1()
    } else {
        variant2()
    }
}

fn variant1() -> impl ToString {
    ", World".to_string()
}

fn variant2() -> impl ToString {
    " World".to_string()
}

fn main() {
    println!("Hello{}!", get_name().to_string());
}
