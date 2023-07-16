plain
travis_time:end:2eecc5a0:start=1557028563534341605,finish=1557028651814816423,duration=88280474818
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:42]    Compiling cmake v0.1.38
[00:27:42]    Compiling backtrace-sys v0.1.27
[00:27:45]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:27:45]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:27:46] error[E0119]: conflicting implementations of trait `convert::TryFrom<&[_]>`:
[00:27:46]     |
[00:27:46]     |
[00:27:46] 106 | impl<T, const N: usize> TryFrom<&[T]> for [T; N] where T: Copy {
[00:27:46]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation
[00:27:46]    ::: src/libcore/convert.rs:565:1
[00:27:46]     |
[00:27:46]     |
[00:27:46] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:46]     | -------------------------------------------- first implementation here
[00:27:46] 
[00:27:46] error[E0119]: conflicting implementations of trait `convert::TryFrom<&[_]>` for type `&[type error]`:
[00:27:46]     |
[00:27:46]     |
[00:27:46] 115 | impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N] {
[00:27:46]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&[type error]`
[00:27:46]    ::: src/libcore/convert.rs:565:1
[00:27:46]     |
[00:27:46]     |
[00:27:46] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:46]     | -------------------------------------------- first implementation here
[00:27:46] 
[00:27:46] error[E0119]: conflicting implementations of trait `convert::TryFrom<&mut [_]>` for type `&mut [type error]`:
[00:27:46]     |
[00:27:46]     |
[00:27:46] 129 | impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N] {
[00:27:46]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut [type error]`
[00:27:46]    ::: src/libcore/convert.rs:565:1
[00:27:46]     |
[00:27:46]     |
[00:27:46] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:46]     | -------------------------------------------- first implementation here
[00:27:46] 
[00:27:46] error[E0119]: conflicting implementations of trait `iter::traits::collect::IntoIterator` for type `&[_; _]`:
[00:27:46]     |
[00:27:46]     |
[00:27:46] 241 | impl<I: Iterator> IntoIterator for I {
[00:27:46]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&[_; _]`
[00:27:46]    ::: src/libcore/array.rs:157:1
[00:27:46]     |
[00:27:46]     |
[00:27:46] 157 | impl<'a, T, const N: usize> IntoIterator for &'a [T; N] {
[00:27:46]     | ------------------------------------------------------- first implementation here
[00:27:46] error: aborting due to 4 previous errors
[00:27:46] 
[00:27:46] For more information about this error, try `rustc --explain E0119`.
[00:27:46] error: Could not compile `core`.
---
156508 ./src/llvm-project/clang
144972 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
144968 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
142416 ./obj/build/bootstrap/debug/incremental/bootstrap-ngvt240vptb4
142412 ./obj/build/bootstrap/debug/incremental/bootstrap-ngvt240vptb4/s-fbx55aqn4o-1jmsec0-li41thz4lrxh
123640 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
97596 ./src/llvm-project/clang/test
89976 ./src/llvm-emscripten/test/CodeGen
