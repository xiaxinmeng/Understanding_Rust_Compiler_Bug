plain
travis_time:end:01335518:start=1550227402203888641,finish=1550227403088153556,duration=884264915
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:08:09]    Compiling tempfile v3.0.5
[00:08:11]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:08:12]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:08:16]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:35] error: internal compiler error: src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:08:35] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:588:9
[00:08:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:08:35] error: aborting due to previous error
[00:08:35] 
[00:08:35] 
[00:08:35] 
[00:08:35] note: the compiler unexpectedly panicked. this is a bug.
[00:08:35] 
[00:08:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:08:35] 
[00:08:35] note: rustc 1.33.0-beta.1 (d1add9723 2019-01-17) running on x86_64-unknown-linux-gnu
[00:08:35] 
[00:08:35] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:08:35] note: some of the compiler flags provided by cargo are hidden
[00:08:35] 
[00:08:35] error: Could not compile `syntax`.
[00:08:35] 
[00:08:35] 
[00:08:35] To learn more, run the command again with --verbose.
[00:08:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:35] expected success, got: exit code: 101
[00:08:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:35] Build completed unsuccessfully in 0:02:24
[00:08:35] make: *** [all] Error 1
[00:08:35] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09e651dc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 10:52:09 UTC 2019
---
199368 ./obj/build/cache/2019-01-18
156148 ./src/llvm-project/clang
155976 ./obj/build/bootstrap/debug/incremental
141204 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e
141200 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e/s-f9icrrc4d9-yihg6n-2ee07hmknei1u
108528 ./src/llvm-project/lldb
97552 ./src/llvm-project/clang/test
93688 ./.git
89964 ./src/llvm-emscripten/test/CodeGen
