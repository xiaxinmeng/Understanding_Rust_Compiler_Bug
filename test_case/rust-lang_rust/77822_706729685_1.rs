rust
#![feature(test)]
#![feature(specialization)]
#![feature(trusted_len)]
#![feature(core_intrinsics)]

#![allow(soft_unstable)]

extern crate test;

pub fn test_1(x: &[u32], y: &[u32]) -> u32 {
    let mut sum = 0;
    let chunk_size = y.len();
    for (c, y) in Enumerate::new(y.iter()) {
        for chunk in x.chunks_exact(chunk_size) {
            sum += chunk[c] + y;
        }
    }
    sum
}

pub fn test_0(x: &[u32], y: &[u32]) -> u32 {
    let mut sum = 0;
    let chunk_size = y.len();
    for (c, y) in y.iter().enumerate() {
        for chunk in x.chunks_exact(chunk_size) {
            sum += chunk[c] + y;
        }
    }
    sum
}

struct Enumerate<I> {
    iter: I,
    count: usize,
    len: usize,
}

impl<I: Iterator> Enumerate<I> {
    #[inline]
    fn new(iter: I) -> Self {
        EnumerateImpl::new(iter)
    }
}

impl<I> Iterator for Enumerate<I>
where
    I: Iterator,
{
    type Item = (usize, <I as Iterator>::Item);

    #[inline]
    fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
        EnumerateImpl::next(self)
    }
}

// Enumerate specialization trait
#[doc(hidden)]
trait EnumerateImpl<I> {
    type Item;
    fn new(iter: I) -> Self;
    fn next(&mut self) -> Option<(usize, Self::Item)>;
}

impl<I> EnumerateImpl<I> for Enumerate<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline]
    default fn new(iter: I) -> Self {
        Enumerate {
            iter,
            count: 0,
            len: 0, // unused
        }
    }

    #[inline]
    default fn next(&mut self) -> Option<(usize, I::Item)> {
        let a = self.iter.next()?;
        let i = self.count;
        // Possible undefined overflow.
        self.count += 1;
        Some((i, a))
    }
}

impl<I> EnumerateImpl<I> for Enumerate<I>
where
    I: std::iter::TrustedLen + ExactSizeIterator + Iterator,
{
    #[inline]
    fn new(iter: I) -> Self {
        let len = iter.size_hint().0;

        Enumerate {
            iter,
            count: 0,
            len,
        }
    }

    #[inline]
    fn next(&mut self) -> Option<(usize, I::Item)> {
        let a = self.iter.next()?;
        unsafe { std::intrinsics::assume(self.count < self.len); }
        let i = self.count;
        // Possible undefined overflow.
        self.count += 1;
        Some((i, a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_test_0(b: &mut Bencher) {
        let x = vec![0; 4 * 1024 * 1024];
        let y = vec![0; 4 * 1024 * 1024];
        b.iter(|| {
            test::black_box(test_0(test::black_box(&x), test::black_box(&y)));
        });
    }

    #[bench]
    fn bench_test_1(b: &mut Bencher) {
        let x = vec![0; 4 * 1024 * 1024];
        let y = vec![0; 4 * 1024 * 1024];
        b.iter(|| {
            test::black_box(test_1(test::black_box(&x), test::black_box(&y)));
        });
    }
}
