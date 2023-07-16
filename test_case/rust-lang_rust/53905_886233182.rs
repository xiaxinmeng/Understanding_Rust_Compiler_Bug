rust
use rayon::prelude::*;

fn main() {
    let mut y = 0;
    (0..10).into_par_iter().for_each(|x| println!("{}{}", x, &mut y));
}
