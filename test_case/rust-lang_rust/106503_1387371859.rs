plain
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-apple-darwin target=x86_64-apple-darwin

failures:

---- [rustdoc] rust/tests/rustdoc/primitive/no_std.rs stdout ----
thread '[rustdoc] rust/tests/rustdoc/primitive/no_std.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 22, kind: InvalidInput, message: "Invalid argument" }', src/tools/compiletest/src/runtest.rs:2352:34


failures:
    [rustdoc] rust/tests/rustdoc/primitive/no_std.rs
