plain
travis_time:end:17748f7c:start=1543912563515235839,finish=1543912632080453342,duration=68565217503
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:38] 
[00:56:38] running 120 tests
[00:56:41] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:56:41] ..ii.i.....iiii.....
[00:56:41] 
[00:56:41]  finished in 3.537
[00:56:41] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:56] 
[00:56:56] running 118 tests
[00:57:21] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:57:25] ......iii.i.....ii
[00:57:25] 
[00:57:25]  finished in 29.022
[00:57:25] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:28] 
[00:57:28] running 60 tests
[00:58:45] .F.FF......................F......FFFF.......FF............test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:00:47] failures:
[01:00:47] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:00:47] 
[01:00:47] ---- [run-pass] run-pass-fulldeps/custom-derive-partial-eq.rs stdout ----
[01:00:47] ---- [run-pass] run-pass-fulldeps/custom-derive-partial-eq.rs stdout ----
[01:00:47] 
[01:00:47] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_partial_eq.rs" failed to compile: 
[01:00:47] status: exit code: 1
[01:00:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_partial_eq.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/custom-derive-partial-eq/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/custom-derive-partial-eq/auxiliary"
[01:00:47] ------------------------------------------
[01:00:47] 
[01:00:47] ------------------------------------------
[01:00:47] stderr:
[01:00:47] stderr:
[01:00:47] ------------------------------------------
[01:00:47] {"message":"cannot find value `mitem` in this scope","code":{"code":"E0425","explanation":"\nAn unresolved name was used.\n\nErroneous code examples:\n\n