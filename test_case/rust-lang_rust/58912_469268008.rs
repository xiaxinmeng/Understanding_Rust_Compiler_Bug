plain
travis_time:end:32d3cb16:start=1551704235150858969,finish=1551704236051714311,duration=900855342
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:34] 
[01:19:34] running 99 tests
[01:19:50] ..............F....................................................................................
[01:19:50] 
[01:19:50] 
[01:19:50] ---- [incremental] incremental/cyclic-trait-hierarchy.rs stdout ----
[01:19:50] 
[01:19:50] error in revision `rpass2`: compilation failed!
[01:19:50] status: exit code: 1
[01:19:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/cyclic-trait-hierarchy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/cyclic-trait-hierarchy/cyclic-trait-hierarchy.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/cyclic-trait-hierarchy/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/cyclic-trait-hierarchy/auxiliary"
[01:19:50] ------------------------------------------
[01:19:50] 
[01:19:50] ------------------------------------------
[01:19:50] stderr:
[01:19:50] stderr:
[01:19:50] ------------------------------------------
[01:19:50] {"message":"cycle detected when computing the supertraits of `T2`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n