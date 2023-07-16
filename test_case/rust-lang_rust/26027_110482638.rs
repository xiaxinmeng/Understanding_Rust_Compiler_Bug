 rust
// run-fail/test-panic.rs

// check-stdout
// error-pattern:thread 'test_foo' panicked at
// compile-flags: --test
// ignore-pretty: does not work well with `--test`

#[test]
fn test_foo() {
    panic!()
}
