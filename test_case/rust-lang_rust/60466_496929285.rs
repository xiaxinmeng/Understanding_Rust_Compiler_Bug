plain
travis_time:end:0d3141d4:start=1559133662428386217,finish=1559133664671129112,duration=2242742895
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:43]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:27:43]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:43]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:27:43]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:54] error: internal compiler error: src/librustc/ty/sty.rs:2311: expected constant usize, got Const {
[00:27:54]     ty: usize,
[00:27:54]     val: Param(
[00:27:54]         N/#1,
[00:27:54] }
[00:27:54] 
[00:27:54] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[00:27:54] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[00:27:54] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:54] 
[00:27:54] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:27:54] 
[00:27:54] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:27:54] note: some of the compiler flags provided by cargo are hidden
[00:27:54] 
[00:27:54] error: Could not compile `core`.
[00:27:54] 
---
travis_time:end:0a162434:start=1559135350478752977,finish=1559135350483476080,duration=4723103
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a1b9750
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:afte
