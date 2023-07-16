rust
extern crate rand;

use rand::{Rng,Thread_rng};

fn main() {
    println!("Hello, world! {}",*thread_rng().choose(&[0, 1, 2, 3]).unwrap());

}
