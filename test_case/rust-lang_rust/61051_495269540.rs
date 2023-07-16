plain
travis_time:end:2d13f48e:start=1558623802914656233,finish=1558623891549134830,duration=88634478597
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:50]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:51]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:27:51]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:52]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:28:16] error: internal compiler error: src/librustc_codegen_ssa/mir/rvalue.rs:405: impossible case reached
[00:28:16] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[00:28:16] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:28:17] error: aborting due to previous error
[00:28:17] 
[00:28:17] 
[00:28:17] 
[00:28:17] note: the compiler unexpectedly panicked. this is a bug.
[00:28:17] 
[00:28:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:28:17] 
[00:28:17] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[00:28:17] 
[00:28:17] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:28:17] note: some of the compiler flags provided by cargo are hidden
[00:28:17] 
[00:28:17] error: Could not compile `core`.
[00:28:17] 
---
19712 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
19660 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
travis_time:end:2b52cbc0:start=1558625598642880951,finish=1558625599290157046,duration=647276095
travis_fold:end:after_failure.1
san/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a9c0cb0
$ dmesg | grep -i kill
