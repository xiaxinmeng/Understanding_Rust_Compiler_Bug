rust
const fn from_fn<F, T, const N: usize>(cb: F) -> [T; N]
where
    F: ~const + FnMut(usize) -> T;
