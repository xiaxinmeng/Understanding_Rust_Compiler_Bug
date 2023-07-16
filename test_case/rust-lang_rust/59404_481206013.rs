plain
travis_time:end:0e4447ee:start=1554806233818091807,finish=1554806236150447893,duration=2332356086
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:26:58]    Compiling cc v1.0.28
[00:26:58]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:26:58]    Compiling libc v0.2.51
[00:26:58]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:26:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[00:26:58] 
[00:26:58] error: internal compiler error: unexpected panic
[00:26:58] 
[00:26:58] note: the compiler unexpectedly panicked. this is a bug.
[00:26:58] note: the compiler unexpectedly panicked. this is a bug.
[00:26:58] 
[00:26:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:58] 
[00:26:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:26:58] 
[00:26:58] note: compiler flags: -Z external-macro-backtrace -Z emit-stack-sizes -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:26:58] note: some of the compiler flags provided by cargo are hidden
[00:26:58] 
[00:26:58] error: Could not compile `core`.
[00:26:58] warning: build failed, waiting for other jobs to finish...
---
198080 ./obj/build/cache/2019-03-20
158160 ./obj/build/bootstrap/debug/incremental
156496 ./src/llvm-project/clang
143236 ./obj/build/bootstrap/debug/incremental/bootstrap-2bzajeq8agxyg
143232 ./obj/build/bootstrap/debug/incremental/bootstrap-2bzajeq8agxyg/s-fb4s6w89x0-1dr8ue2-8d51newq7bun
139608 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
136768 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123640 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
---
travis_time:end:0f3fe46e:start=1554807872437898657,finish=1554807872444112612,duration=6213955
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04011084
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0165fb3c
travis_time:start:0165fb3c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/bui
