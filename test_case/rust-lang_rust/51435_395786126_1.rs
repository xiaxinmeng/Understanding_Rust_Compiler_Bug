rust
#![feature(try_trait, test)]

extern crate test;

use std::ops::Try;

use test::{Bencher, black_box};

#[no_mangle]
pub fn std_compute(a: u64, b: u64) -> u64 {
    StepByWithoutTryFold(StepBy {
        iter: a..b,
        step: 6,
        first_take: true,
    }).map(|x| x ^ (x - 1)).sum()
}

#[no_mangle]
pub fn pr_compute(a: u64, b: u64) -> u64 {
    StepBy {
        iter: a..b,
        step: 6,
        first_take: true,
    }.map(|x| x ^ (x - 1)).sum()
}

#[bench]
fn std_bench(bencher: &mut Bencher) {
    let a = black_box(1);
    let b = black_box(5000000);
    bencher.iter(|| {
        black_box(std_compute(a, b));
    });
}

#[bench]
fn pr_bench(bencher: &mut Bencher) {
    let a = black_box(1);
    let b = black_box(5000000);
    bencher.iter(|| {
        black_box(pr_compute(a, b));
    });
}

struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}

struct StepByWithoutTryFold<I>(StepBy<I>);

impl<I: Iterator> Iterator for StepBy<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.first_take {
            self.first_take = false;
            self.iter.next()
        } else {
            self.iter.nth(self.step)
        }
    }

    #[inline]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    {
        let mut accum = init;

        if self.first_take {
            self.first_take = false;
            if let Some(x) = self.iter.next() {
                accum = f(accum, x)?;
            } else {
                return Try::from_ok(accum);
            }
        }

        while let Some(x) = self.iter.nth(self.step) {
            accum = f(accum, x)?;
        }
        Try::from_ok(accum)
    }
}

impl<I: Iterator> Iterator for StepByWithoutTryFold<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.first_take {
            self.0.first_take = false;
            self.0.iter.next()
        } else {
            self.0.iter.nth(self.0.step)
        }
    }
}
