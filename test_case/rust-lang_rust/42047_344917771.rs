rust
pub fn foo() -> u64 {
    let mut x = 0;
    // Doesn't currently propagate destination because of the borrow.
    drop(&mut x);
    x
}
