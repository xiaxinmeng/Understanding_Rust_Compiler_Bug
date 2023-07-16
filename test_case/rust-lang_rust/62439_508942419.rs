rust
use std::collections::HashSet;

const N_ITEMS: usize = 50;
const N_REPEATS: usize = 10000001;

fn main() {
    let mut set = HashSet::new();

    for i in 0..N_ITEMS {
        set.insert(i ^ 13);
    }

    let mut res = 0;

    for _ in 0..N_REPEATS {
        for item in set.iter() {
            res ^= item;
        }
    }

    println!("sum: {}", res);
}
