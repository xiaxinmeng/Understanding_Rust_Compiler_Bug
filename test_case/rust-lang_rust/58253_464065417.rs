plain
travis_time:end:04da8834:start=1550239029457929779,finish=1550239101201201061,duration=71743271282
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:18:18]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:18:25]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:18:25]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:18:27]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:18:29] error: internal compiler error: src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:18:29] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:588:9
[00:18:29] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:18:29] error: aborting due to previous error
[00:18:29] 
[00:18:29] 
[00:18:29] 
[00:18:29] note: the compiler unexpectedly panicked. this is a bug.
[00:18:29] 
[00:18:29] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:18:29] 
[00:18:29] note: rustc 1.33.0-beta.1 (d1add9723 2019-01-17) running on x86_64-unknown-linux-gnu
[00:18:29] 
[00:18:29] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:18:29] note: some of the compiler flags provided by cargo are hidden
[00:18:29] 
[00:18:29] error: Could not compile `rustc_resolve`.
[00:18:29] warning: build failed, waiting for other jobs to finish...
[00:18:29] warning: build failed, waiting for other jobs to finish...
[00:19:18] error: build failed
[00:19:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:18] expected success, got: exit code: 101
[00:19:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:18] Build completed unsuccessfully in 0:14:40
[00:19:18] Makefile:18: recipe for target 'all' failed
[00:19:18] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a977b3e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 14:17:49 UTC 2019
---
travis_time:end:0b47266e:start=1550240269733801535,finish=1550240269738179873,duration=4378338
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23c00786
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07dd3bd0
travis_time:start:07dd3bd0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:37aa30a9
$ dmesg | grep -i kill
