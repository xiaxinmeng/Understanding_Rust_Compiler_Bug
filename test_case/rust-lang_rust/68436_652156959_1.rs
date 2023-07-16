rust
const fn double(n: usize) -> usize { 2*n }
fn foo<const N: usize>() {
    let _: [u8; { double(N) }] = ...;
}
