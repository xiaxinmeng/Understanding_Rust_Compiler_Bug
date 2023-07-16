plain
travis_time:end:044ff124:start=1540436660191971954,finish=1540436714688758259,duration=54496786305
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:06:44]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:48]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:09]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:08:19]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:32] error: value assigned to `base_ty` is never read
[00:09:32]    --> librustc/mir/tcx.rs:310:13
[00:09:32]     |
[00:09:32] 310 |             base_ty = next_ty;
[00:09:32]     |
[00:09:32]     = note: `-D unused-assignments` implied by `-D warnings`
[00:09:32] 
[00:09:32] error: aborting due to previous error
[00:09:32] error: aborting due to previous error
[00:09:32] 
[00:09:32] error: Could not compile `rustc`.
[00:09:32] 
[00:09:32] To learn more, run the command again with --verbose.
[00:09:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:32] expected success, got: exit code: 101
[00:09:32] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:09:32] travis_fold:end:stage0-rustc

[00:09:32] travis_time:end:stage0-rustc:start=1540437074746020681,finish=1540437296372182127,duration=221626161446


[00:09:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:09:32] Build completed unsuccessfully in 0:04:42
[00:09:32] make: *** [all] Error 1
[00:09:32] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0393d066
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
