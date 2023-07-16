rust
#![feature(test)]

extern crate rand;
extern crate test;

use rand::Rng;
use test::Bencher;

#[inline(never)]
pub fn foo(a: Option<usize>, b: Option<usize>) -> usize {
    if let (Some(a), Some(b)) = (a, b) {
        a + b
    } else {
        0
    }
}

#[inline(never)]
pub fn bar(a: Option<usize>, b: Option<usize>) -> usize {
    if a.is_some() && b.is_some() {
        a.unwrap() + b.unwrap()
    } else {
        0
    }
}

#[inline(never)]
pub fn baz(a: Option<usize>, b: Option<usize>) -> usize {
    if let Some(a) = a {
        if let Some(b) = b {
            return a + b;
        }
    }
    0
}

#[inline(never)]
fn bench(b: &mut Bencher, f: fn(Option<usize>, Option<usize>) -> usize) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed);
    let data : Vec<_> = rng.gen_iter::<(Option<usize>, Option<usize>)>().take(1000).collect();
    b.iter(|| {
        data.iter().fold(0, |acc, &(a, b)| acc + f(a, b))
    });
}

#[bench]
fn bench_foo(b: &mut Bencher) {
    bench(b, foo)
}

#[bench]
fn bench_bar(b: &mut Bencher) {
    bench(b, bar)
}

#[bench]
fn bench_baz(b: &mut Bencher) {
    bench(b, baz)
}
