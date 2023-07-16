
#![crate_type = "lib"]

extern crate test;
use test::Bencher;

// These two functions are exactly the same.
pub fn clone_str_bad(x: &String, bh: &mut Bencher) {
    bh.iter(|| x.clone());
}
pub fn clone_str_good(x: &String, bh: &mut Bencher) {
    bh.iter(|| x.clone());
}
