plain
test [rustdoc] src/test/rustdoc/wrapping.rs ... ok

failures:

---- [rustdoc] src/test/rustdoc/primitive/primitive-generic-impl.rs stdout ----
thread '[rustdoc] src/test/rustdoc/primitive/primitive-generic-impl.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 22, kind: InvalidInput, message: "Invalid argument" }', src/tools/compiletest/src/runtest.rs:125:43
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-apple-darwin target=x86_64-apple-darwin


failures:
