plain
travis_time:end:08567c70:start=1544690910733210007,finish=1544690997613604566,duration=86880394559
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
[00:53:57] 
[00:53:57] running 121 tests
[00:54:00] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.......ii..i..i.ii..............i...i 100/121
[00:54:00] i..ii.i.....iiii.....
[00:54:00] 
[00:54:00]  finished in 3.361
[00:54:00] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:14] 
[00:54:14] running 119 tests
[00:54:36] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[00:54:39] i......iii.i.....ii
[00:54:39] 
[00:54:39]  finished in 25.667
[00:54:39] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:16] 
[00:55:16] running 60 tests
[00:56:44] ..................F..............FFFFF.....FFFFF....F....F.test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[00:58:22] failures:
[00:58:23] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:58:23] 
[00:58:23] ---- [run-pass] run-pass-fulldeps/issue-16723.rs stdout ----
[00:58:23] ---- [run-pass] run-pass-fulldeps/issue-16723.rs stdout ----
[00:58:23] 
[00:58:23] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs" failed to compile: 
[00:58:23] status: exit code: 1
[00:58:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-16723/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-16723/auxiliary"
[00:58:23] ------------------------------------------
[00:58:23] 
[00:58:23] ------------------------------------------
[00:58:23] stderr:
[00:58:23] stderr:
[00:58:23] ------------------------------------------
[00:58:23] {"message":"unused import: `smallvec::SmallVec`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/issue-16723.rs","byte_start":730,"byte_end":748,"line_start":23,"line_end":23,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"use smallvec::SmallVec;","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacementments the trait, and there could\ntheoretically be an infinite number of types.\n\nFor example, with:\n\n