plain
travis_time:end:033313b0:start=1549827015950859489,finish=1549827018560751280,duration=2609891791
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:56:07]     Checking term v0.0.0 (/checkout/src/libterm)
[00:56:07]     Checking getopts v0.2.17
[00:56:07]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:56:11]  Documenting test v0.0.0 (/checkout/src/libtest)
[00:56:11] error: internal compiler error: src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:56:11] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:595:9
[00:56:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:56:11] 
[00:56:11] error: Unrecognized option: 'crate-version'
[00:56:11] error: Unrecognized option: 'crate-version'
[00:56:11] 
[00:56:11] error: Could not document `test`.
[00:56:11] 
[00:56:11] Caused by:
[00:56:11]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name test src/libtest/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-2cd31f015e79e3a8.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9b2503eb72968b11.rmeta --extern term=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libterm-0bd89f567a02c34d.rmeta` (exit code: 1)
[00:56:11] 
[00:56:11] 
[00:56:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test"
[00:56:11] 
[00:56:11] 
[00:56:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:56:11] Build completed unsuccessfully in 0:06:22
[00:56:11] Build completed unsuccessfully in 0:06:22
[00:56:11] make: *** [all] Error 1
[00:56:11] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05fa7902
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 20:26:40 UTC 2019
---
travis_time:end:041324dd:start=1549830401909376098,finish=1549830401914822768,duration=5446670
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05cd9178
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travi
