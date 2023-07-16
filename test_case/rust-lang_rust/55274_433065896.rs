plain
travis_time:end:00433623:start=1540476004754957098,finish=1540476061798990958,duration=57044033860
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:07:14]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:17] error[E0425]: cannot find value `el` in this scope
[00:07:17]   --> librustc/mir/tcx.rs:83:38
[00:07:17]    |
[00:07:17] 83 |         self.projection_ty_core(tcx, el, |_, _, ty| ty)
[00:07:17]    |                                      ^^ not found in this scope
[00:07:43] error: aborting due to previous error
[00:07:43] 
[00:07:43] For more information about this error, try `rustc --explain E0425`.
[00:07:44] error: Could not compile `rustc`.
[00:07:44] error: Could not compile `rustc`.
[00:07:44] 
[00:07:44] To learn more, run the command again with --verbose.
[00:07:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:44] expected success, got: exit code: 101
[00:07:44] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:07:44] travis_fold:end:stage0-rustc

[00:07:44] travis_time:end:stage0-rustc:start=1540476372763269974,finish=1540476535946552647,duration=163183282673


[00:07:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:44] Build completed unsuccessfully in 0:03:39
[00:07:44] make: *** [all] Error 1
[00:07:44] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01ac38e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151412 ./src/tools/clang
149628 ./obj/build/bootstrap/debug/incremental
149120 ./src/llvm-emscripten/test
134172 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
134168 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f61x03xniz-fn1ppn-1wbz4rzqyzyro
107668 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
93748 ./src/tools/clang/test
87540 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
