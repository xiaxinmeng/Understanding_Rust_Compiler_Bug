plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
test header::tests::asm_support ... FAILED

failures:

---- header::tests::channel stdout ----
thread 'header::tests::channel' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::llvm_version stdout ----
---- header::tests::llvm_version stdout ----
thread 'header::tests::llvm_version' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::aux_build stdout ----
---- header::tests::aux_build stdout ----
thread 'header::tests::aux_build' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::no_system_llvm stdout ----
error: test failed, to rerun pass '--bin compiletest'
error: test failed, to rerun pass '--bin compiletest'
thread 'header::tests::no_system_llvm' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::debugger stdout ----
---- header::tests::debugger stdout ----
thread 'header::tests::debugger' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::ignore_target stdout ----
---- header::tests::ignore_target stdout ----
thread 'header::tests::ignore_target' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::should_fail stdout ----
---- header::tests::should_fail stdout ----
thread 'header::tests::should_fail' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::stage stdout ----
---- header::tests::stage stdout ----
thread 'header::tests::stage' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::sanitizers stdout ----
---- header::tests::sanitizers stdout ----
thread 'header::tests::sanitizers' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::revisions stdout ----
---- header::tests::revisions stdout ----
thread 'header::tests::revisions' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::test_duplicate_revisions stdout ----
---- header::tests::test_duplicate_revisions stdout ----
thread 'header::tests::test_duplicate_revisions' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
note: panic did not contain expected string
      panic message: `"called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"`,
 expected substring: `"Duplicate revision: `rpass1` in line ` rpass1 rpass1`"`
---- header::tests::cross_compile stdout ----
thread 'header::tests::cross_compile' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::only_target stdout ----
---- header::tests::only_target stdout ----
thread 'header::tests::only_target' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10
---- header::tests::asm_support stdout ----
---- header::tests::asm_support stdout ----
thread 'header::tests::asm_support' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/compiletest/src/main.rs:219:10

failures:
    header::tests::asm_support
    header::tests::aux_build
