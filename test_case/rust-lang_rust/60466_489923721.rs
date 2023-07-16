plain
travis_time:end:02cc2a1f:start=1557208150405668102,finish=1557208151158370166,duration=752702064
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:52]    Compiling compiler_builtins v0.1.10
[00:27:52]    Compiling cmake v0.1.38
[00:27:52]    Compiling backtrace-sys v0.1.27
[00:27:54]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:27:56] error[E0119]: conflicting implementations of trait `convert::TryFrom<&[_]>`:
[00:27:56]     |
[00:27:56]     |
[00:27:56] 106 | impl<T, const N: usize> TryFrom<&[T]> for [T; N] where T: Copy {
[00:27:56]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation
[00:27:56]    ::: src/libcore/convert.rs:565:1
[00:27:56]     |
[00:27:56]     |
[00:27:56] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:56]     | -------------------------------------------- first implementation here
[00:27:56] 
[00:27:56] error[E0119]: conflicting implementations of trait `convert::TryFrom<&[_]>` for type `&[type error]`:
[00:27:56]     |
[00:27:56]     |
[00:27:56] 115 | impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N] {
[00:27:56]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&[type error]`
[00:27:56]    ::: src/libcore/convert.rs:565:1
[00:27:56]     |
[00:27:56]     |
[00:27:56] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:56]     | -------------------------------------------- first implementation here
[00:27:56] 
[00:27:56] error[E0119]: conflicting implementations of trait `convert::TryFrom<&mut [_]>` for type `&mut [type error]`:
[00:27:56]     |
[00:27:56]     |
[00:27:56] 129 | impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N] {
[00:27:56]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut [type error]`
[00:27:56]    ::: src/libcore/convert.rs:565:1
[00:27:56]     |
[00:27:56]     |
[00:27:56] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:56]     | -------------------------------------------- first implementation here
[00:27:56] 
[00:27:56] error[E0119]: conflicting implementations of trait `iter::traits::collect::IntoIterator` for type `&[_; _]`:
[00:27:56]     |
[00:27:56]     |
[00:27:56] 241 | impl<I: Iterator> IntoIterator for I {
[00:27:56]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&[_; _]`
[00:27:56]    ::: src/libcore/array.rs:157:1
[00:27:56]     |
[00:27:56]     |
[00:27:56] 157 | impl<'a, T, const N: usize> IntoIterator for &'a [T; N] {
[00:27:56]     | ------------------------------------------------------- first implementation here
[00:27:56]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:27:56]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:56] error: aborting due to 4 previous errors
[00:27:56] 
---
19252 ./src/llvm-project/lldb/www/cpp_reference/html
18164 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen
17940 ./src/llvm-project/lldb/www/python_reference
travis_time:end:177a9f8c:start=1557209842834305831,finish=1557209843411631209,duration=57t ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04e03f4f
$ dmesg | grep -i kill
