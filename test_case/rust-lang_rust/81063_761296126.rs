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
test runtest::tests::normalize_platform_differences ... ok

failures:

---- header::tests::ignore_target stdout ----
thread 'header::tests::ignore_target' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19

---- header::tests::debugger stdout ----
---- header::tests::debugger stdout ----
thread 'header::tests::debugger' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::only_target stdout ----
---- header::tests::only_target stdout ----
thread 'header::tests::only_target' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::revisions stdout ----
---- header::tests::revisions stdout ----
thread 'header::tests::revisions' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::no_system_llvm stdout ----
---- header::tests::no_system_llvm stdout ----
thread 'header::tests::no_system_llvm' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::cross_compile stdout ----
---- header::tests::cross_compile stdout ----
thread 'header::tests::cross_compile' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::llvm_version stdout ----
---- header::tests::llvm_version stdout ----
thread 'header::tests::llvm_version' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::aux_build stdout ----
---- header::tests::aux_build stdout ----
thread 'header::tests::aux_build' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::stage stdout ----
---- header::tests::stage stdout ----
thread 'header::tests::stage' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::should_fail stdout ----
---- header::tests::should_fail stdout ----
thread 'header::tests::should_fail' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19
---- header::tests::sanitizers stdout ----
---- header::tests::sanitizers stdout ----
thread 'header::tests::sanitizers' panicked at 'OptionMissing("jsondocck-path")', src/tools/compiletest/src/main.rs:154:19

failures:
    header::tests::aux_build
    header::tests::cross_compile
