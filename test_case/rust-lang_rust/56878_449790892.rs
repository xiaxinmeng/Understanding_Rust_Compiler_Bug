plain
travis_time:end:05786c38:start=1545703077104426157,finish=1545703078182545183,duration=1078119026
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:43:23]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:44:25]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:44:48]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:44:48]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:46:06] error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(59/0:1991 ~ rustc_target[2d6f]::spec[0]::load_specific[0])
[00:46:06] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:600:9
[00:46:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:46:06] error: aborting due to previous error
[00:46:06] 
[00:46:06] 
[00:46:06] 
[00:46:06] note: the compiler unexpectedly panicked. this is a bug.
[00:46:06] 
[00:46:06] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:46:06] 
[00:46:06] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:46:06] 
[00:46:06] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:46:06] note: some of the compiler flags provided by cargo are hidden
[00:46:06] 
[00:46:06] error: Could not compile `rustc_driver`.
[00:46:06] warning: build failed, waiting for other jobs to finish...
[00:46:06] warning: build failed, waiting for other jobs to finish...
[00:46:12] error: build failed
[00:46:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:46:12] expected success, got: exit code: 101
[00:46:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:46:12] Build completed unsuccessfully in 0:42:56
[00:46:12] make: *** [all] Error 1
[00:46:12] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:293dc192
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 25 02:44:18 UTC 2018
