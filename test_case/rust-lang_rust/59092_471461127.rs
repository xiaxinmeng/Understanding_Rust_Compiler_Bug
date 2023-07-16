plain
travis_time:end:064184fe:start=1552294150705271146,finish=1552294151637714759,duration=932443613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:27:16]    Compiling cmake v0.1.33
[00:27:16]    Compiling backtrace-sys v0.1.27
[00:27:17] thread 'rustc' panicked at 'assertion failed: `(left == right)`
[00:27:17]   left: `1`,
[00:27:17]  right: `0`', src/librustc/ich/impls_hir.rs:793:17
[00:27:17] 
[00:27:17] error: internal compiler error: unexpected panic
[00:27:17] 
[00:27:17] note: the compiler unexpectedly panicked. this is a bug.
[00:27:17] note: the compiler unexpectedly panicked. this is a bug.
[00:27:17] 
[00:27:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:17] 
[00:27:17] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:27:17] 
[00:27:17] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:27:17] note: some of the compiler flags provided by cargo are hidden
[00:27:17] 
[00:27:17] error: Could not compile `core`.
[00:27:17] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:25d8657e:start=1552295802389197042,finish=1552295802394137888,duration=4940846
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2d36c9c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1286d496
travis_time:start:1286d496
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0362ed08
$ dmesg | grep -i kill
