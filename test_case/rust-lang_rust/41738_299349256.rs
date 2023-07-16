rust
#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use test::Bencher;

    #[bench]
    fn bench_pop_front(b: &mut Bencher) {
        let size: usize = 1_000_000;
        let mut zeros: VecDeque<u8> = VecDeque::with_capacity(size);
        for i in 0..size {
            zeros.push_front(0u8);
        }

        b.iter(|| {
            for i in 0..size {
                zeros.pop_front();
            }
        })
    }

    #[bench]
    fn bench_pop_back(b: &mut Bencher) {
        let size: usize = 1_000_000;
        let mut zeros: VecDeque<u8> = VecDeque::with_capacity(size);
        for i in 0..size {
            zeros.push_back(0u8);
        }

        b.iter(|| {
            for i in 0..size {
                zeros.pop_back();
            }
        })
    }
}
