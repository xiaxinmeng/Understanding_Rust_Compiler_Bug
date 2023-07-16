plain
travis_time:end:054f9488:start=1550269907233521994,finish=1550269908072747248,duration=839225254
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:48]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:58]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:39]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:39]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:41] error[E0425]: cannot find value `impl_node_id` in this scope
[00:12:41]    --> src/librustc_typeck/check/compare_method.rs:845:56
[00:12:41]     |
[00:12:41] 845 |                                     if param.hir_id == impl_node_id {
[00:12:41]     |                                                        ^^^^^^^^^^^^ help: a local variable with a similar name exists: `impl_def_id`
[00:12:46] error: aborting due to previous error
[00:12:46] 
[00:12:46] For more information about this error, try `rustc --explain E0425`.
[00:12:47] error: Could not compile `rustc_typeck`.
[00:12:47] error: Could not compile `rustc_typeck`.
[00:12:47] warning: build failed, waiting for other jobs to finish...
[00:16:46] error: build failed
[00:16:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:46] expected success, got: exit code: 101
[00:16:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:46] Build completed unsuccessfully in 0:13:04
[00:16:46] make: *** [all] Error 1
[00:16:46] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e7516f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 22:48:47 UTC 2019
---
199368 ./obj/build/cache/2019-01-18
156148 ./src/llvm-project/clang
155976 ./obj/build/bootstrap/debug/incremental
141204 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e
141200 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e/s-f9iw89h4bv-1kw3swl-2ee07hmknei1u
123516 ./src/llvm-project/llvm/test/CodeGen
108528 ./src/llvm-project/lldb
102208 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
102204 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
