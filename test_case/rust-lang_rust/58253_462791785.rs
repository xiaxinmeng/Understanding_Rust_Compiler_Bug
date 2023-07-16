plain
travis_time:end:00cba17a:start=1549980389985089744,finish=1549980392154684231,duration=2169594487
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:52:00]    Compiling parking_lot_core v0.3.0
[00:52:02]    Compiling parking_lot v0.6.4
[00:52:04]    Compiling tempfile v3.0.5
[00:52:05]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:52:06] thread 'rustc' panicked at 'src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:52:06] 
[00:52:06] error: internal compiler error: unexpected panic
[00:52:06] 
[00:52:06] note: the compiler unexpectedly panicked. this is a bug.
---
[00:52:06] 
[00:52:06] 
[00:52:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:52:06] Build completed unsuccessfully in 0:43:46
[00:52:06] make: *** [all] Error 1
[00:52:06] Makefile:18: recipe for target 'all' failed
100764 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
97552 ./src/llvm-project/clang/test
92556 ./.git
89964 ./src/llvm-emscripten/test/CodeGen
