plain
travis_time:end:2aa7c640:start=1547618085075328614,finish=1547618087207846588,duration=2132517974
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
[01:10:59] 
[01:10:59] running 118 tests
[01:11:23] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:11:27] ......iii.i.....ii
[01:11:27] 
[01:11:27]  finished in 27.834
[01:11:27] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:06] 
[01:12:06] running 60 tests
[01:14:11] ..............................................F............test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:16:24] failures:
[01:16:24] 
[01:16:24] ---- [run-pass] run-pass-fulldeps/pprust-expr-roundtrip.rs stdout ----
[01:16:24] 
[01:16:24] 
[01:16:24] error: test compilation failed although it shouldn't!
[01:16:24] status: exit code: 1
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n