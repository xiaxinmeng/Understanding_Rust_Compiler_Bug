plain
travis_time:end:0ec977b3:start=1559063966082370690,finish=1559063968357003172,duration=2274632482
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:35:05] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:35:05] 
[00:35:05] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:35:05] 
[00:35:05] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:35:05] note: some of the compiler flags provided by cargo are hidden
[00:35:05] 
[00:35:05] error: Could not compile `rustc`.
[00:35:05] 
---
travis_time:end:1089e4cc:start=1559066085396457995,finish=1559066085404705015,duration=8247020
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dd644d7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b58b698
$ dmesg | grep -i kill
