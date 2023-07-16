plain
travis_time:end:01e02fbb:start=1556982930778490906,finish=1556983019455123521,duration=88676632615
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:21]    Compiling cmake v0.1.38
[00:27:21]    Compiling backtrace-sys v0.1.27
[00:27:25]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:27:25]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:25] error[E0119]: conflicting implementations of trait `convert::TryFrom<&[_]>`:
[00:27:25]     |
[00:27:25]     |
[00:27:25] 106 | impl<T, const N: usize> TryFrom<&[T]> for [T; N] where T: Copy {
[00:27:25]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation
[00:27:25]    ::: src/libcore/convert.rs:565:1
[00:27:25]     |
[00:27:25]     |
[00:27:25] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:25]     | -------------------------------------------- first implementation here
[00:27:25] 
[00:27:25] error[E0119]: conflicting implementations of trait `convert::TryFrom<&[_]>` for type `&[type error]`:
[00:27:25]     |
[00:27:25]     |
[00:27:25] 115 | impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N] {
[00:27:25]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&[type error]`
[00:27:25]    ::: src/libcore/convert.rs:565:1
[00:27:25]     |
[00:27:25]     |
[00:27:25] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:25]     | -------------------------------------------- first implementation here
[00:27:25] 
[00:27:25] error[E0119]: conflicting implementations of trait `convert::TryFrom<&mut [_]>` for type `&mut [type error]`:
[00:27:25]     |
[00:27:25]     |
[00:27:25] 129 | impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N] {
[00:27:25]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut [type error]`
[00:27:25]    ::: src/libcore/convert.rs:565:1
[00:27:25]     |
[00:27:25]     |
[00:27:25] 565 | impl<T, U> TryFrom<U> for T where U: Into<T> {
[00:27:25]     | -------------------------------------------- first implementation here
[00:27:25] 
[00:27:25] error[E0119]: conflicting implementations of trait `iter::traits::collect::IntoIterator` for type `&[_; _]`:
[00:27:25]     |
[00:27:25]     |
[00:27:25] 241 | impl<I: Iterator> IntoIterator for I {
[00:27:25]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&[_; _]`
[00:27:25]    ::: src/libcore/array.rs:157:1
[00:27:25]     |
[00:27:25]     |
[00:27:25] 157 | impl<'a, T, const N: usize> IntoIterator for &'a [T; N] {
[00:27:25]     | ------------------------------------------------------- first implementation here
[00:27:25] error: aborting due to 4 previous errors
[00:27:25] 
[00:27:25] For more information about this error, try `rustc --explain E0119`.
[00:27:25] error: Could not compile `core`.
---
travis_time:end:1261a65e:start=1556984679997518335,finish=1556984680002602124,duration=5083789
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07894bd8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21159d78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/
