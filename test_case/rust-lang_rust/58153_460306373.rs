plain
travis_time:end:2a2cb7b0:start=1549295283505722831,finish=1549295289797450337,duration=6291727506
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:15:44]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:46] error[E0308]: mismatched types
[00:15:46]    --> src/librustc_metadata/encoder.rs:677:58
[00:15:46]     |
[00:15:46] 677 |         let variant_data = tcx.hir().expect_variant_data(variant_id);
[00:15:46]     |                                                          ^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[00:15:46]     = note: expected type `rustc::hir::HirId`
[00:15:46]                found type `syntax::ast::NodeId`
[00:15:46] 
[00:15:48] error: aborting due to previous error
---
[00:20:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:20:58] expected success, got: exit code: 101
[00:20:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:58] Build completed unsuccessfully in 0:14:41
[00:20:58] Makefile:18: recipe for target 'all' failed
[00:20:58] make: *** [all] Error 1
24792 ./src/llvm-project/compiler-rt/test/builtins/Unit
24128 ./src/llvm-project/lldb/packages
24124 ./src/llvm-project/lldb/packages/Python
24120 ./src/llvm-project/lldb/packages/Python/lldbsuite
---
travis_time:end:04f1cbee:start=1549296558444535015,finish=1549296559040536449,duration=596001434
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:09e1b688
$ ls -lat $HOME/Library/Logs/Diagnosti/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e34b287
$ dmesg | grep -i kill
