plain
travis_time:end:32d63af5:start=1552113615040351762,finish=1552113615929897456,duration=889545694
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:57:35]    Compiling parking_lot_core v0.4.0
[00:57:40]    Compiling tempfile v3.0.5
[00:57:42]    Compiling parking_lot v0.7.1
[00:57:43]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:57:48] error[E0599]: no method named `first` found for type `std::str::Chars<'_>` in the current scope
[00:57:48]     |
[00:57:48]     |
[00:57:48] 302 |             if ori_link.chars().first().unwrap().is_numeric() {
[00:57:48] 
[00:57:49] error: aborting due to previous error
[00:57:49] 
[00:57:49] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:05a780d6:start=1552117096005320376,finish=1552117096012714702,duration=7394326
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0337a408
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:066356d0
$ dmesg | grep -i kill
