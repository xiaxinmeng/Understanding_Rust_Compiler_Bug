plain
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 203 tests
..............................................F.....................F................... 88/203
...........................
failures:

---- spec::tests::arm64_32_apple_watchos stdout ----
---- spec::tests::arm64_32_apple_watchos stdout ----
thread 'spec::tests::arm64_32_apple_watchos' panicked at 'assertion failed: self.dynamic_linking || self.position_independent_executables', compiler/rustc_target/src/spec/tests/tests_impl.rs:137:13

---- spec::tests::armv7k_apple_watchos stdout ----
---- spec::tests::armv7k_apple_watchos stdout ----
thread 'spec::tests::armv7k_apple_watchos' panicked at 'assertion failed: self.dynamic_linking || self.position_independent_executables', compiler/rustc_target/src/spec/tests/tests_impl.rs:137:13

failures:
failures:
error: test failed, to rerun pass `-p rustc_target --lib`
    spec::tests::armv7k_apple_watchos

test result: FAILED. 201 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

