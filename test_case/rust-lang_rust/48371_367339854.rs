
extern crate rand;

use rand::{Rng,SeedableRng,XorShiftRng};

fn main() {
    let mut rng:XorShiftRng = SeedableRng::from_seed([1,2,3,4]);
    let mut m:f64 = 0.0;
    for _ in 0..1_000_000_000 {
        let x = rng.gen();
        if x > m { m = x; }
    }
    println!("{}", m);
}
