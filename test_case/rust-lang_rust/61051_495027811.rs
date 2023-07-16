plain
travis_time:end:0054f939:start=1558571451160397763,finish=1558571453268513802,duration=2108116039
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:44]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:27:44]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:44]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:45]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:28:10] error: internal compiler error: src/librustc_codegen_ssa/mir/place.rs:405: using operand local ((*_1).0: str::SplitInternal<str::IsWhitespace>) as place
[00:28:10] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[00:28:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:28:11] error: aborting due to previous error
[00:28:11] 
[00:28:11] 
[00:28:11] 
[00:28:11] note: the compiler unexpectedly panicked. this is a bug.
[00:28:11] 
[00:28:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:28:11] 
[00:28:11] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[00:28:11] 
[00:28:11] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:28:11] note: some of the compiler flags provided by cargo are hidden
[00:28:11] 
[00:28:11] error: Could not compile `core`.
[00:28:11] 
---
156496 ./src/llvm-project/clang
145304 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
145300 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
142472 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9
142468 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9/s-fcgtx7f6hk-1pwjdzc-11lco0fc7150k
123648 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
102240 ./.git
97584 ./src/llvm-project/clang/test
---
travis_time:end:0b8f6d80:start=1558573156142840844,finish=1558573156148855290,duration=6014446
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:154813e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b14fa2a
