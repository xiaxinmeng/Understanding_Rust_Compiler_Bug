plain
travis_time:end:0f835704:start=1549461450672988741,finish=1549461453750530783,duration=3077542042
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:44] 
[01:24:44] running 119 tests
[01:25:11] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:25:15] i......iii.i.....ii
[01:25:15] 
[01:25:15]  finished in 31.776
[01:25:15] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:23] 
[01:25:23] running 47 tests
[01:26:57] ......................................F.......test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:29:18] .
[01:29:18] failures:
[01:29:18] 
[01:29:18] ---- [run-pass] run-pass-fulldeps/pprust-expr-roundtrip.rs stdout ----
[01:29:18] ---- [run-pass] run-pass-fulldeps/pprust-expr-roundtrip.rs stdout ----
[01:29:18] 
[01:29:18] error: test compilation failed although it shouldn't!
[01:29:18] status: exit code: 1
[01:29:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/auxiliary"
[01:29:18] ------------------------------------------
[01:29:18] 
[01:29:18] ------------------------------------------
[01:29:18] stderr:
[01:29:18] stderr:
[01:29:18] ------------------------------------------
[01:29:18] {"message":"cannot borrow immutable argument `e` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n