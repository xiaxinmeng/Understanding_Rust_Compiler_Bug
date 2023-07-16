 rust
#![feature(macro_rules)]

use std::collections::{HashMap,HashSet};

macro_rules! seq {
    ($($x:expr),+) => {
        [$($x,)+].iter().map(|&x| x).collect()
    }
}

fn main() {
    let set: HashSet<int> = seq!(1, 2, 3);
    let map: HashMap<int, int> = seq!((1, 10), (2, 20), (3, 30));

    println!("Set: {}", set);
    println!("Map: {}", map);
}
