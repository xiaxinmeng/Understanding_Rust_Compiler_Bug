plain
travis_time:end:05f75b98:start=1551441462969877667,finish=1551441465377474016,duration=2407596349
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:21:17]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:22:19]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:22:19]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:22:19]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:23:26] thread 'rustc' panicked at 'src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:23:26] 
[00:23:26] error: internal compiler error: unexpected panic
[00:23:26] 
[00:23:26] note: the compiler unexpectedly panicked. this is a bug.
[00:23:26] note: the compiler unexpectedly panicked. this is a bug.
[00:23:26] 
[00:23:26] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:23:26] 
[00:23:26] note: rustc 1.33.0-beta.7 (08f107300 2019-02-16) running on x86_64-unknown-linux-gnu
[00:23:26] 
[00:23:26] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:23:26] note: some of the compiler flags provided by cargo are hidden
[00:23:26] 
[00:23:26] error: Could not compile `rustc_driver`.
[00:23:26] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:00da045a:start=1551442900819052416,finish=1551442900826752242,duration=7699826
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:055fdf43
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e6b113
travis_time:start:01e6b113
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dy
