plain
travis_time:end:27491cb8:start=1545414406348127381,finish=1545414461271821386,duration=54923694005
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:46] 
[00:24:46] error: missing documentation for macro
[00:24:46]   --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:46:1
[00:24:46]    |
[00:24:46] 46 | macro_rules! is_aarch64_feature_detected {
[00:24:46] 
[00:24:46] error: missing documentation for macro
[00:24:46]   --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:65:1
[00:24:46]    |
[00:24:46]    |
[00:24:46] 65 | macro_rules! is_powerpc_feature_detected {
[00:24:46] 
[00:24:46] error: missing documentation for macro
[00:24:46]   --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:84:1
[00:24:46]    |
[00:24:46]    |
[00:24:46] 84 | macro_rules! is_powerpc64_feature_detected {
[00:24:46] 
[00:24:46] error: missing documentation for macro
[00:24:46]    --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:103:1
[00:24:46]     |
[00:24:46]     |
[00:24:46] 103 | macro_rules! is_mips_feature_detected {
[00:24:46] 
[00:24:46] error: missing documentation for macro
[00:24:46]    --> src/libstd/../stdsimd/stdsimd/arch/detect/error_macros.rs:122:1
[00:24:46]     |
[00:24:46]     |
[00:24:46] 122 | macro_rules! is_mips64_feature_detected {
[00:24:46] 
[00:24:46] error: aborting due to 8 previous errors
[00:24:46] 
[00:24:46] error: Could not compile `std`.
[00:24:46] error: Could not compile `std`.
[00:24:46] 
[00:24:46] To learn more, run the command again with --verbose.
[00:24:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:24:46] expected success, got: exit code: 101
[00:24:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:24:46] Build completed unsuccessfully in 0:21:26
[00:24:46] Makefile:28: recipe for target 'all' failed
[00:24:46] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24734be8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 18:12:36 UTC 2018
