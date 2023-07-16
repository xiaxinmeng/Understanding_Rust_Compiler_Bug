rust
#![feature(test)]

extern crate test;
extern crate rand;

use test::Bencher;
use rand::Rng;

#[bench]
fn hashmap_insert(b: &mut Bencher) {
    let mut val : u32 = 0;
    let mut rng = rand::thread_rng();
    let mut map = std::collections::HashMap::new();

    b.iter(|| { map.insert(rng.gen::<u8>() as usize, val); val += 1; })
}
