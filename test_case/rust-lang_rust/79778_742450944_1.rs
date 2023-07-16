rust
const fn bar(n: usize) -> usize {
    assert!(cake(n));
    n
}
fn foo<const N: usize>(val: Val) -> [u8; bar(N)] where [u8; bar(N)]: Sized { .. }
