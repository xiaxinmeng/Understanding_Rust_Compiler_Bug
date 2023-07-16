 rust
#[cfg(test)] fn f() {}
#[cfg(test)] macro_rules! m { () => {} }

#[test]
fn test() {
    f(); //< this is OK,
    m!(); //< but this is currently an error (on non-test builds).
}
