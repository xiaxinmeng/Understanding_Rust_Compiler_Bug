plain
travis_time:end:2312b483:start=1542755280460092720,finish=1542755281902444787,duration=1442352067
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:17:49]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:18:03] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[00:18:03]    --> librustc_codegen_llvm/attributes.rs:199:5
[00:18:03]     |
[00:18:03] 64  |   fn unwind(val: &'ll Value, can_unwind: bool) {
[00:18:03]     |   -------------------------------------------- defined here
[00:18:03] ...
[00:18:03] 199 | /     unwind(if cx.tcx.sess.panic_strategy() != PanicStrategy::Unwind {
[00:18:03] 200 | |         // In panic=abort mode we assume nothing can unwind anywhere, so
[00:18:03] 201 | |         // optimize based on this!
[00:18:03] ...   |
[00:18:03] 228 | |         true
[00:18:03] 229 | |     });
[00:18:03]     | |______^ expected 2 parameters
---
145100 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
145096 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
145092 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134660 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6v028rh0j-1jh577i-2q1xp1m83q6vz
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
93748 ./src/tools/clang/test
89976 ./src/llvm-emscripten/test/CodeGen
