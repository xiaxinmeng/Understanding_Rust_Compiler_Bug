 rust
#![feature(test, read_exact)]
extern crate rand;
extern crate test;

use std::io::prelude::*;
use std::fs;
use rand::Rng;

#[bench]
fn os_rng_u32(b: &mut test::Bencher) {
    b.iter(|| {
        rand::OsRng::new().unwrap().gen::<u32>();
    })
}

#[bench]
fn read_dev_urandom_u32(b: &mut test::Bencher) {
    b.iter(|| {
        let mut b = [0; 4];
        fs::File::open("/dev/urandom").unwrap().read_exact(&mut b).unwrap();
    })
}

#[bench]
fn sort_0050(b: &mut test::Bencher) {
    let v = rand::thread_rng().gen_iter::<u32>().take(50).collect::<Vec<_>>();
    b.iter(|| {
        v.clone().sort();
    })
}
#[bench]
fn sort_0100(b: &mut test::Bencher) {
    let v = rand::thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
    b.iter(|| {
        v.clone().sort();
    })
}
#[bench]
fn sort_0500(b: &mut test::Bencher) {
    let v = rand::thread_rng().gen_iter::<u32>().take(500).collect::<Vec<_>>();
    b.iter(|| {
        v.clone().sort();
    })
}

#[bench]
fn sort_1000(b: &mut test::Bencher) {
    let v = rand::thread_rng().gen_iter::<u32>().take(1000).collect::<Vec<_>>();
    b.iter(|| {
        v.clone().sort();
    })
}

