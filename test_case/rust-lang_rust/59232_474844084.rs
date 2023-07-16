plain
travis_time:end:01f36748:start=1553089040798012072,finish=1553089130633943536,duration=89835931464
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:26:34]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:26:35]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:26:35]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:26:35]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:00] thread 'rustc' panicked at 'assertion failed: mir.arg_count == 0', src/librustc_mir/const_eval.rs:148:5
[00:27:00] 
[00:27:00] error: internal compiler error: unexpected panic
[00:27:00] 
[00:27:00] note: the compiler unexpectedly panicked. this is a bug.
[00:27:00] note: the compiler unexpectedly panicked. this is a bug.
[00:27:00] 
[00:27:00] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:00] 
[00:27:00] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:27:00] 
[00:27:00] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:27:00] note: some of the compiler flags provided by cargo are hidden
[00:27:00] 
[00:27:00] error: Could not compile `core`.
[00:27:00] 
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:00e65944
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access '/hotravis_time:start:0cd3510e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:28194a04
$ dmesg | grep -i kill
