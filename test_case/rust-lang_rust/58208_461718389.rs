plain
travis_time:end:03950d87:start=1549608425122404099,finish=1549608427479329176,duration=2356925077
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:29]     Checking term v0.0.0 (/checkout/src/libterm)
[01:00:29]     Checking getopts v0.2.17
[01:00:29]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:00:33]  Documenting test v0.0.0 (/checkout/src/libtest)
[01:00:33] error: internal compiler error: src/librustc/hir/def.rs:258: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[01:00:33] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:595:9
[01:00:33] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:00:33] 
[01:00:33] error: Unrecognized option: 'crate-version'
[01:00:33] error: Unrecognized option: 'crate-version'
[01:00:33] 
[01:00:33] error: Could not document `test`.
[01:00:33] 
[01:00:33] Caused by:
[01:00:33]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name test src/libtest/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-2cd31f015e79e3a8.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9b2503eb72968b11.rmeta --extern term=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libterm-0bd89f567a02c34d.rmeta` (exit code: 1)
[01:00:33] 
[01:00:33] 
[01:00:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test"
[01:00:33] 
[01:00:33] 
[01:00:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:00:33] Build completed unsuccessfully in 0:06:44
[01:00:33] Build completed unsuccessfully in 0:06:44
[01:00:33] make: *** [all] Error 1
[01:00:33] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01aa0758
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 07:47:51 UTC 2019
