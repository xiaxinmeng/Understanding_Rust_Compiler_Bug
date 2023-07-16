rust
pub fn f<T: Copy>(a: T) {
    g(a, a)
}

#[inline(never)]
pub fn g<T: Copy>(a: T, b: T) {}
