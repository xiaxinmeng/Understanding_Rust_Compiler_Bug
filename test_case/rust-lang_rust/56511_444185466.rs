plain
travis_time:end:0a288700:start=1543940861197950177,finish=1543940915006299051,duration=53808348874
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
[00:56:11] 
[00:56:11] running 120 tests
[00:56:14] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:56:14] ..ii.i.....iiii.....
[00:56:14] 
[00:56:14]  finished in 3.469
[00:56:14] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:29] 
[00:56:29] running 118 tests
[00:56:52] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:56] ......iii.i.....ii
[00:56:56] 
[00:56:56]  finished in 27.263
[00:56:56] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:59] 
[00:56:59] running 60 tests
[00:58:13] ..FFFFF...F.FF....F.....F.FF.F...FF.FF.......FF............test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:00:10] failures:
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass-fulldeps/custom-derive-partial-eq.rs stdout ----
[01:00:10] 
[01:00:10] 
[01:00:10] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_partial_eq.rs" failed to compile: 
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_partial_eq.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/custom-derive-partial-eq/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/custom-derive-partial-eq/auxiliary"
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] ------------------------------------------
[01:00:10] stderr:
[01:00:10] stderr:
[01:00:10] ------------------------------------------
[01:00:10] {"message":"cannot find value `mitem` in this scope","code":{"code":"E0425","explanation":"\nAn unresolved name was used.\n\nErroneous code examples:\n\n