rust
fn foo<T: Copy, const N: usize>(a: [T; N]) -> [T; N] {
    a
}
fn main() {
    let SPAM: [(u32, u32); 1] = foo([(1, 1)]);
}
