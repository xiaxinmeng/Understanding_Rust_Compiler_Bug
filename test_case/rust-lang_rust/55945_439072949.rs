plain
travis_time:end:012f6537:start=1542293740011405310,finish=1542293794140951248,duration=54129545938
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:06:59]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:07:09]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:44]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:44]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:12:45] error[E0428]: the name `_` is defined multiple times
[00:12:45]   |
[00:12:45]   = note: `_` must be defined only once in the value namespace of this module
[00:12:45] 
[00:12:45] error[E0428]: the name `_` is defined multiple times
[00:12:45]   |
[00:12:45]   = note: `_` must be defined only once in the value namespace of this module
[00:12:45] 
[00:12:45] error[E0428]: the name `_` is defined multiple times
[00:12:45]   |
[00:12:45]   = note: `_` must be defined only once in the value namespace of this module
[00:12:45] error: aborting due to previous error
[00:12:45] 
[00:12:45] For more information about this error, try `rustc --explain E0428`.
[00:12:45] error: Could not compile `rustc_incremental`.
---
[00:12:58] 
[00:12:58] To learn more, run the command again with --verbose.
[00:12:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:12:58] expected success, got: exit code: 101
[00:12:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:12:58] travis_fold:end:stage0-rustc

[00:12:58] travis_time:end:stage0-rustc:start=1542294094969952935,finish=1542294582869087564,duration=487899134629


[00:12:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:12:58] Build completed unsuccessfully in 0:09:13
[00:12:58] Makefile:28: recipe for target 'all' failed
[00:12:58] make: *** [all] Error 1
