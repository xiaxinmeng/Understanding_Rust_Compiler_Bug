
#![feature(try_trait, test, step_trait)]
use std::iter::Step;

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
fn specialized_pr_bench(bencher: &mut Bencher) {
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

impl<T> Iterator for StepByWithoutTryFold<std::ops::Range<T>>
where
    T: Step,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.first_take = false;
        if self.0.iter.start >= self.0.iter.end {
            return None;
        }
        // add 1 to self.step to get original step size back
        // it was decremented for the general case on construction
        if let Some(n) = self.0.iter.start.add_usize(self.0.step+1) {
            let next = std::mem::replace(&mut self.0.iter.start, n);
            Some(next)
        } else {
            let last = self.0.iter.start.clone();
            self.0.iter.start = self.0.iter.end.clone();
            Some(last)
        }
    }
}
