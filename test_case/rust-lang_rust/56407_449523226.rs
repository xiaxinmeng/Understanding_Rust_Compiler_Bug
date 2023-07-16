plain
travis_time:end:0e414298:start=1545433573163335160,finish=1545433626701910478,duration=53538575318
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:23:35]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:23:35]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:23:35]    Compiling rustc-demangle v0.1.10
[00:23:41]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
0m65 | macro_rules! is_powerpc_feature_detected {
[00:23:49] 
[00:23:49] error: missing documentation for macro
[00:23:49]   --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:84:1
[00:23:49]    |
[00:23:49]    |
[00:23:49] 84 | macro_rules! is_powerpc64_feature_detected {
[00:23:49] 
[00:23:49] error: missing documentation for macro
[00:23:49]    --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:103:1
[00:23:49]     |
[00:23:49]     |
[00:23:49] 103 | macro_rules! is_mips_feature_detected {
[00:23:49] 
[00:23:49] error: missing documentation for macro
[00:23:49]    --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:122:1
[00:23:49]     |
[00:23:49]     |
[00:23:49] 122 | macro_rules! is_mips64_feature_detected {
[00:23:49] 
[00:23:49] error: aborting due to 6 previous errors
[00:23:49] 
[00:23:49] error: Could not compile `std`.
[00:23:49] error: Could not compile `std`.
[00:23:49] 
[00:23:49] To learn more, run the command again with --verbose.
[00:23:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:49] expected success, got: exit code: 101
[00:23:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:49] Build completed unsuccessfully in 0:20:45
[00:23:49] Makefile:28: recipe for target 'all' failed
[00:23:49] make: *** [all] Error 1
