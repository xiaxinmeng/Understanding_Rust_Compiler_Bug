rust
const A: usize = b();
const fn b() -> usize {
    let _: usize = A;
    42
}
