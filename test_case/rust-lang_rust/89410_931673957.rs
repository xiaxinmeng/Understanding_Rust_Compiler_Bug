rust
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    println!("{}", rng.next_u64());
}
