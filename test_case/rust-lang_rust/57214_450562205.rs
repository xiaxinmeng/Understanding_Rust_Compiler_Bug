plain
travis_time:end:001bd734:start=1546176078948784479,finish=1546176079992216741,duration=1043432262
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:23:40]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:23:41]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:23:41]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:23:41]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:23:42] thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/libcore/slice/mod.rs:2455:10
[00:23:42] 
[00:23:42] error: internal compiler error: unexpected panic
[00:23:42] 
[00:23:42] note: the compiler unexpectedly panicked. this is a bug.
[00:23:42] note: the compiler unexpectedly panicked. this is a bug.
[00:23:42] 
[00:23:42] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:23:42] 
[00:23:42] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:23:42] 
[00:23:42] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:23:42] note: some of the compiler flags provided by cargo are hidden
[00:23:42] 
[00:23:42] error: Could not compile `core`.
[00:23:42] warning: build failed, waiting for other jobs to finish...
[00:23:42] warning: build failed, waiting for other jobs to finish...
[00:23:44] error: build failed
[00:23:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:44] expected success, got: exit code: 101
[00:23:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:44] Build completed unsuccessfully in 0:20:41
[00:23:44] Makefile:18: recipe for target 'all' failed
[00:23:44] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01abab4a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 30 13:45:13 UTC 2018
---
travis_time:end:033ecb14:start=1546177513793309586,finish=1546177513799080850,duration=5771264
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03527b52
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b9c8c78
travis_time:start:1b9c8c78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15015924
$ dmesg | grep -i kill
