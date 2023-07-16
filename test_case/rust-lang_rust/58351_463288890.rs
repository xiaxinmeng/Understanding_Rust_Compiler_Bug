plain
travis_time:end:02bbcc50:start=1550073528021582722,finish=1550074103376909768,duration=575355327046
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:15:56] failures:
[01:15:56] 
[01:15:56] ---- [run-pass] run-pass/statics/static-recursive.rs stdout ----
[01:15:56] 
[01:15:56] error: test compilation failed although it shouldn't!
[01:15:56] status: exit code: 1
[01:15:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/statics/static-recursive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/statics/static-recursive/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/statics/static-recursive/auxiliary"
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] stderr:
[01:15:56] stderr:
[01:15:56] ------------------------------------------
[01:15:56] {"message":"cycle detected when const-evaluating `L1`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n