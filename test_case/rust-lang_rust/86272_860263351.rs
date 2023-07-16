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
test header::tests::debugger ... FAILED

failures:

---- header::tests::llvm_version stdout ----
thread 'header::tests::llvm_version' panicked at 'assertion failed: parse_rs(&config, \"// min-llvm-version: 9.0\").ignore', src/tools/compiletest/src/header/tests.rs:126:5
---- header::tests::no_system_llvm stdout ----
---- header::tests::no_system_llvm stdout ----
thread 'header::tests::no_system_llvm' panicked at 'assertion failed: parse_rs(&config, \"// no-system-llvm\").ignore', src/tools/compiletest/src/header/tests.rs:118:5
---- header::tests::stage stdout ----
---- header::tests::stage stdout ----
thread 'header::tests::stage' panicked at 'assertion failed: parse_rs(&config, \"// ignore-stage1\").ignore', src/tools/compiletest/src/header/tests.rs:177:5
---- header::tests::channel stdout ----
---- header::tests::channel stdout ----
thread 'header::tests::channel' panicked at 'assertion failed: parse_rs(&config, \"// ignore-beta\").ignore', src/tools/compiletest/src/header/tests.rs:243:5
error: test failed, to rerun pass '--bin compiletest'
---- header::tests::ignore_target stdout ----
---- header::tests::ignore_target stdout ----
thread 'header::tests::ignore_target' panicked at 'assertion failed: parse_rs(&config, \"// ignore-x86_64-unknown-linux-gnu\").ignore', src/tools/compiletest/src/header/tests.rs:143:5
---- header::tests::cross_compile stdout ----
---- header::tests::cross_compile stdout ----
thread 'header::tests::cross_compile' panicked at 'assertion failed: parse_rs(&config, \"// ignore-cross-compile\").ignore', src/tools/compiletest/src/header/tests.rs:186:5
---- header::tests::sanitizers stdout ----
---- header::tests::sanitizers stdout ----
thread 'header::tests::sanitizers' panicked at 'assertion failed: parse_rs(&config, \"// needs-sanitizer-address\").ignore', src/tools/compiletest/src/header/tests.rs:221:5
---- header::tests::asm_support stdout ----
---- header::tests::asm_support stdout ----
thread 'header::tests::asm_support' panicked at 'assertion failed: parse_rs(&config, \"// needs-asm-support\").ignore', src/tools/compiletest/src/header/tests.rs:232:5
---- header::tests::only_target stdout ----
---- header::tests::only_target stdout ----
thread 'header::tests::only_target' panicked at 'assertion failed: parse_rs(&config, \"// only-i686\").ignore', src/tools/compiletest/src/header/tests.rs:160:5
---- header::tests::debugger stdout ----
---- header::tests::debugger stdout ----
thread 'header::tests::debugger' panicked at 'assertion failed: parse_rs(&config, \"// ignore-cdb\").ignore', src/tools/compiletest/src/header/tests.rs:199:5

failures:
    header::tests::asm_support
    header::tests::channel
