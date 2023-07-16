
use std::num::Zero;

struct PrefixSum<T, A> {
    iter: T,
    sum: A
}

trait MyAdditiveIterator<A> {
    fn prefix_sum(self) -> PrefixSum<Self, A>;
}

impl<A: Add<A, A> + Zero, T: Iterator<A>> MyAdditiveIterator<A> for T {
    #[inline]
    fn prefix_sum(self) -> PrefixSum<T, A> {
        PrefixSum { iter: self, sum: Zero::zero() }
    }
}

impl<A: Add<A, A> + Zero + Clone, T: Iterator<A>> Iterator<A> for PrefixSum<T, A> {
    fn next(&mut self) -> Option<A> {
        match self.iter.next() {
            Some(x) => { self.sum = self.sum.add(&x); Some(self.sum.clone()) }
            None => None
        }
    }
}

fn main() {
    let x: Vec<uint> = vec![1, 2, 3, 4, 5];
    for i in x.iter().map(|&x| x).prefix_sum() {
        println!("{}", i);
    }
}
