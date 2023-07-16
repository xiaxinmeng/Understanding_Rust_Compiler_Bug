rust
// Problem: playground uses rand_pcg v0.2.1 crate which is outdated and depends
// on rand_core 0.5, while rand v0.8.4 depends on rand_core 0.6, hence two
// versions of the SeedableRng trait exist.

use rand::{Rng, SeedableRng};

fn main() {
    let mut rng = rand_pcg::Pcg32::seed_from_u64(123);
    println!("{}", rng.gen::<i32>());
}
