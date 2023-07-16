plain
travis_time:end:103b74e5:start=1552982833695344552,finish=1552982915182693725,duration=81487349173
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:26:27]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:26:28]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:26:28]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:26:28]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:26:52] thread 'rustc' panicked at 'assertion failed: mir.arg_count == 0', src/librustc_mir/const_eval.rs:148:5
[00:26:52] 
[00:26:52] error: internal compiler error: unexpected panic
[00:26:52] 
[00:26:52] note: the compiler unexpectedly panicked. this is a bug.
[00:26:52] note: the compiler unexpectedly panicked. this is a bug.
[00:26:52] 
[00:26:52] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:52] 
[00:26:52] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:26:52] 
[00:26:52] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:26:52] note: some of the compiler flags provided by cargo are hidden
[00:26:52] 
[00:26:52] error: Could not compile `core`.
[00:26:52] 
---
travis_time:end:059edb98:start=1552984537897927364,finish=1552984537902533285,duration=4605921
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014c7970
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b9eaf1e
travis_time:start:1b9eaf1e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01ad8d3f
$ dmesg | grep -i kill
