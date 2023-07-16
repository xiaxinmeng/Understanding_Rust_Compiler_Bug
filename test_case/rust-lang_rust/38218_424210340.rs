rust
use std::time::SystemTime;

fn main() {
    let n = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() % 1000;
    let mut x = 0;

    for i in 0..n {
        x += i;
    }

    println!("{}", x);
}
