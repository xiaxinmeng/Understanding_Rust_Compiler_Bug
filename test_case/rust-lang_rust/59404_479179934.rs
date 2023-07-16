plain
travis_time:end:037f7ffa:start=1554233306954260655,finish=1554233400863200049,duration=93908939394
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:43:22]    Compiling cc v1.0.28
[00:43:22]    Compiling libc v0.2.51
[00:43:22]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:43:22]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:43:22] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[00:43:22] 
[00:43:22] error: internal compiler error: unexpected panic
[00:43:22] 
[00:43:22] note: the compiler unexpectedly panicked. this is a bug.
[00:43:22] note: the compiler unexpectedly panicked. this is a bug.
[00:43:22] 
[00:43:22] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:43:22] 
[00:43:22] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:43:22] 
[00:43:22] note: compiler flags: -Z external-macro-backtrace -Z emit-stack-sizes -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:43:22] note: some of the compiler flags provided by cargo are hidden
[00:43:22] 
[00:43:22] error: Could not compile `core`.
[00:43:22] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:07bd13d2:start=1554236018305673385,finish=1554236018310228787,duration=4555402
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:035b81e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:029b7d7e
travis_time:start:029b7d7e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05cd34a8
$ dmesg | grep -i kill
