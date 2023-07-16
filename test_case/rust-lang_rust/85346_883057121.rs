plain
Mismatch at tests/target/fn.rs:33:
 {
 }
 
-fn foo<U: Fn(A) -> B /* paren inside generics */>() {}
+fn foo<U: Fn(A) -> B /* paren inside generics */>(A) -> B /* paren inside generics */>() {}
 impl Foo {
 impl Foo {
     fn with_no_errors<T, F>(&mut self, f: F) -> T
test test::idempotence_tests ... FAILED

failures:


---- test::idempotence_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity=Crate` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
  left: `1`,
 right: `0`: 1 idempotent tests failed', src/tools/rustfmt/src/test/mod.rs:324:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'test::idempotence_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:74:10

failures:
    test::idempotence_tests

