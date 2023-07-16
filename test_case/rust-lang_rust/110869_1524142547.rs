plain
..............................F.......

failures:

---- spec::tests::aarch64_unknown_uefi stdout ----
thread 'spec::tests::aarch64_unknown_uefi' panicked at 'assertion failed: `(left matches right)`
  left: `Unix(Yes)`,
 right: `LinkerFlavor::Msvc(..)`', compiler/rustc_target/src/spec/tests/tests_impl.rs:57:25

---- spec::tests::i686_unknown_uefi stdout ----
---- spec::tests::i686_unknown_uefi stdout ----
thread 'spec::tests::i686_unknown_uefi' panicked at 'assertion failed: `(left matches right)`
error: test failed, to rerun pass `-p rustc_target --lib`
  left: `Unix(Yes)`,
 right: `LinkerFlavor::Msvc(..)`', compiler/rustc_target/src/spec/tests/tests_impl.rs:57:25
---- spec::tests::x86_64_unknown_uefi stdout ----
---- spec::tests::x86_64_unknown_uefi stdout ----
thread 'spec::tests::x86_64_unknown_uefi' panicked at 'assertion failed: `(left matches right)`
  left: `Unix(Yes)`,
 right: `LinkerFlavor::Msvc(..)`', compiler/rustc_target/src/spec/tests/tests_impl.rs:57:25

failures:
    spec::tests::aarch64_unknown_uefi
    spec::tests::i686_unknown_uefi
