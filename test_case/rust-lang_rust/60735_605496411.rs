Rust
#![feature(const_generics)]

fn const_chunks_exact<T, const N: usize>(slice: &[T]) -> ConstChunksExact<'_, T, { N }> {
        assert!(N != 0);
        let rem = slice.len() % N;
        let len = slice.len() - rem;
        let (fst, snd) = slice.split_at(len);
        ConstChunksExact::<T, { N }> { v: fst, rem: snd }
    }

struct ConstChunksExact<'a, T: 'a, const N: usize> {
    v: &'a [T],
    rem: &'a [T],
}

impl<'a, T: 'a, const N: usize> Iterator for ConstChunksExact<'a, T, { N }> {
    type Item = &'a [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.v.len() < N {
            None
        } else {
            let (fst, snd) = self.v.split_at(N);

            self.v = snd;

            let ptr = fst.as_ptr() as *const [T; N];

            // SAFETY: ok because we just checked that the length fits
            unsafe { Some(&*ptr) }
        }
    }
}

fn main() {
    let mut slice = const_chunks_exact::<_, 3>(&[1, 2, 3, 4, 5, 6i32]);

    for [a, b, c] in slice {
        dbg!(a, b, c);
    }
}

