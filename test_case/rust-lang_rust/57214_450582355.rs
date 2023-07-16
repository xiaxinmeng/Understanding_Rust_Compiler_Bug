plain
travis_time:end:0eabcb1d:start=1546197131636217777,finish=1546197196655265062,duration=65019047285
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:23:34]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:23:34]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:23:35]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:23:35]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:23:39] thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 9', /checkout/src/libcore/slice/mod.rs:2455:10
[00:23:39] 
[00:23:39] error: internal compiler error: unexpected panic
[00:23:39] 
[00:23:39] note: the compiler unexpectedly panicked. this is a bug.
[00:23:39] note: the compiler unexpectedly panicked. this is a bug.
[00:23:39] 
[00:23:39] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:23:39] 
[00:23:39] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:23:39] 
[00:23:39] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:23:39] note: some of the compiler flags provided by cargo are hidden
[00:23:39] 
[00:23:39] error: Could not compile `core`.
[00:23:39] 
[00:23:39] 
[00:23:39] To learn more, run the command again with --verbose.
[00:23:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:39] expected success, got: exit code: 101
[00:23:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:39] Build completed unsuccessfully in 0:20:40
[00:23:39] Makefile:18: recipe for target 'all' failed
[00:23:39] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06ef39e1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 30 19:37:05 UTC 2018
---
travis_time:end:03321de0:start=1546198626062880647,finish=1546198626067744601,duration=4863954
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a844914
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00abd1e8
travis_time:start:00abd1e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a891124
$ dmesg | grep -i kill
