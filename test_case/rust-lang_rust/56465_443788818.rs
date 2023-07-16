plain
travis_time:end:1838f24d:start=1543856779456768316,finish=1543856780730150664,duration=1273382348
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:50]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:59] warning: method is never used: `map_err_into`
[00:03:59]    --> src/libcore/result.rs:913:5
[00:03:59]     |
[00:03:59] 913 | /     fn map_err_into<F>(self) -> Result<T, F>
[00:03:59] 915 | |         E: Into<F>
[00:03:59] 916 | |     {
[00:03:59] 916 | |     {
[00:03:59] 917 | |         self.map_err(E::into)
[00:03:59]     | |_____^
[00:03:59]     |
[00:03:59]     = note: #[warn(dead_code)] on by default
[00:03:59] 
[00:03:59] 
[00:03:59] error: This node does not have a stability attribute
[00:03:59]    --> src/libcore/result.rs:903:5
[00:03:59]     |
[00:03:59] 903 | /     pub fn map_into<U>(self) -> Result<U, E>
[00:03:59] 905 | |         T: Into<U>
[00:03:59] 906 | |     {
[00:03:59] 906 | |     {
[00:03:59] 907 | |         self.map(T::into)
[00:03:59]     | |_____^
[00:03:59] 
[00:03:59] error: aborting due to previous error
[00:03:59] 
[00:03:59] 
[00:03:59] error: Could not compile `core`.
[00:03:59] 
[00:03:59] To learn more, run the command again with --verbose.
[00:03:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:59] expected success, got: exit code: 101
[00:03:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:59] Build completed unsuccessfully in 0:00:22
[00:03:59] make: *** [all] Error 1
[00:03:59] Makefile:28: recipe for target 'all' failed
41660 ./src/llvm/lib/Target
41264 ./src/llvm-emscripten/test/CodeGen/X86
40496 ./src/llvm-emscripten/lib/Target
37776 ./src/tools/lldb/www
