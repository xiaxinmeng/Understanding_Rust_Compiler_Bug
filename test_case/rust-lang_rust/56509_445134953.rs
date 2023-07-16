plain
travis_time:end:1a3f10b0:start=1544161593915828770,finish=1544161664792372575,duration=70876543805
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:29:15]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:29:15]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:29:21]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:30:51]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:31:03] thread 'main' panicked at 'type must lift when substs do', src/libcore/option.rs:1008:5
[00:31:04] 
[00:31:04] error: internal compiler error: unexpected panic
[00:31:04] 
[00:31:04] note: the compiler unexpectedly panicked. this is a bug.
[00:31:04] note: the compiler unexpectedly panicked. this is a bug.
[00:31:04] 
[00:31:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:31:04] 
[00:31:04] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:31:04] 
[00:31:04] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:31:04] note: some of the compiler flags provided by cargo are hidden
[00:31:04] 
[00:31:04] error: Could not compile `rustc`.
[00:31:04] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:196f9801:start=1544163555915219709,finish=1544163555920186128,duration=4966419
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06c35ab6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fac
