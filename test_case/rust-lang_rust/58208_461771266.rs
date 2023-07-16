plain
travis_time:end:11a24972:start=1549621213132329780,finish=1549621215707190484,duration=2574860704
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:57:06]     Checking term v0.0.0 (/checkout/src/libterm)
[00:57:06]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:57:06]     Checking getopts v0.2.17
[00:57:09]  Documenting test v0.0.0 (/checkout/src/libtest)
[00:57:09] error: internal compiler error: src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:57:09] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:595:9
[00:57:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:57:09] 
[00:57:09] error: Unrecognized option: 'crate-version'
[00:57:09] error: Unrecognized option: 'crate-version'
[00:57:09] 
[00:57:09] error: Could not document `test`.
[00:57:09] 
[00:57:09] Caused by:
[00:57:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name test src/libtest/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-2cd31f015e79e3a8.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9b2503eb72968b11.rmeta --extern term=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libterm-0bd89f567a02c34d.rmeta` (exit code: 1)
[00:57:09] 
[00:57:09] 
[00:57:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test"
[00:57:09] 
[00:57:09] 
[00:57:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:57:09] Build completed unsuccessfully in 0:06:13
[00:57:09] Build completed unsuccessfully in 0:06:13
[00:57:09] make: *** [all] Error 1
[00:57:09] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0515b050
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 11:17:35 UTC 2019
