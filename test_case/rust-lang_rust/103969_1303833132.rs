plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---

failures:

---- header::tests::should_fail stdout ----
thread 'header::tests::should_fail' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19

---- header::tests::revisions stdout ----
---- header::tests::revisions stdout ----
thread 'header::tests::revisions' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::aux_build stdout ----
---- header::tests::aux_build stdout ----
thread 'header::tests::aux_build' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::ignore_target stdout ----
---- header::tests::ignore_target stdout ----
thread 'header::tests::ignore_target' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::llvm_version stdout ----
---- header::tests::llvm_version stdout ----
thread 'header::tests::llvm_version' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::no_system_llvm stdout ----
---- header::tests::no_system_llvm stdout ----
thread 'header::tests::no_system_llvm' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::stage stdout ----
---- header::tests::stage stdout ----
thread 'header::tests::stage' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::debugger stdout ----
---- header::tests::debugger stdout ----
thread 'header::tests::debugger' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::only_target stdout ----
---- header::tests::only_target stdout ----
thread 'header::tests::only_target' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::cross_compile stdout ----
---- header::tests::cross_compile stdout ----
thread 'header::tests::cross_compile' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::sanitizers stdout ----
---- header::tests::sanitizers stdout ----
thread 'header::tests::sanitizers' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::asm_support stdout ----
---- header::tests::asm_support stdout ----
thread 'header::tests::asm_support' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::channel stdout ----
error: test failed, to rerun pass `--bin compiletest`
error: test failed, to rerun pass `--bin compiletest`
thread 'header::tests::channel' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::test_duplicate_revisions stdout ----
---- header::tests::test_duplicate_revisions stdout ----
thread 'header::tests::test_duplicate_revisions' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
note: panic did not contain expected string
      panic message: `"OptionMissing(\"sysroot-base\")"`,
 expected substring: `"Duplicate revision: `rpass1` in line ` rpass1 rpass1`"`
---- header::tests::is_big_endian stdout ----
thread 'header::tests::is_big_endian' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::ignore_arch stdout ----
---- header::tests::ignore_arch stdout ----
thread 'header::tests::ignore_arch' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::matches_abi stdout ----
---- header::tests::matches_abi stdout ----
thread 'header::tests::matches_abi' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::matches_os stdout ----
---- header::tests::matches_os stdout ----
thread 'header::tests::matches_os' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::wasm_special stdout ----
---- header::tests::wasm_special stdout ----
thread 'header::tests::wasm_special' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::families stdout ----
---- header::tests::families stdout ----
thread 'header::tests::families' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::pointer_width stdout ----
---- header::tests::pointer_width stdout ----
thread 'header::tests::pointer_width' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19
---- header::tests::matches_env stdout ----
---- header::tests::matches_env stdout ----
thread 'header::tests::matches_env' panicked at 'OptionMissing("sysroot-base")', src/tools/compiletest/src/main.rs:165:19

failures:
    header::tests::asm_support
    header::tests::aux_build
