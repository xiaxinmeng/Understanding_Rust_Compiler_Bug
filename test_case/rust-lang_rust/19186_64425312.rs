 rust
struct Map<A, B, I, F> where I: Iterator<A>, F: FnMut(A) -> B {
    iter: I,
    f: F,
}
