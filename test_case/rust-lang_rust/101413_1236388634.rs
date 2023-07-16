plain
F....................................................................................... 176/197
..............F......
failures:

---- spec::tests::aarch64_unknown_uefi stdout ----
thread 'spec::tests::aarch64_unknown_uefi' panicked at 'assertion failed: self.dynamic_linking || self.position_independent_executables', compiler/rustc_target/src/spec/tests/tests_impl.rs:150:13

---- spec::tests::i686_unknown_uefi stdout ----
---- spec::tests::i686_unknown_uefi stdout ----
thread 'spec::tests::i686_unknown_uefi' panicked at 'assertion failed: self.dynamic_linking || self.position_independent_executables', compiler/rustc_target/src/spec/tests/tests_impl.rs:150:13
---- spec::tests::x86_64_unknown_uefi stdout ----
---- spec::tests::x86_64_unknown_uefi stdout ----
thread 'spec::tests::x86_64_unknown_uefi' panicked at 'assertion failed: self.dynamic_linking || self.position_independent_executables', compiler/rustc_target/src/spec/tests/tests_impl.rs:150:13

failures:
    spec::tests::aarch64_unknown_uefi
    spec::tests::i686_unknown_uefi
    spec::tests::i686_unknown_uefi
    spec::tests::x86_64_unknown_uefi

test result: FAILED. 194 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustc_target --lib'
