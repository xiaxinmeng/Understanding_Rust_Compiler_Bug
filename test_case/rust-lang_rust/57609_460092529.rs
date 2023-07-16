plain
travis_time:end:054b8674:start=1549225061883766319,finish=1549225168997055868,duration=107113289549
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
[01:10:48] 
[01:10:48] running 119 tests
[01:11:12] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:16] i......iii.i.....ii
[01:11:16] 
[01:11:16]  finished in 28.245
[01:11:16] travis_fold:end:test_debuginfo

---
[01:36:19] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0510 (line 8603) stdout ----
[01:36:19] error[E0506]: cannot assign to `x` because it is borrowed
[01:36:19]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8608:18
[01:36:19]   |
[01:36:19] 7 |     Some(v) if { x = None; false } => (),
[01:36:19]   |          |       |
[01:36:19]   |          |       |
[01:36:19]   |          |       assignment to borrowed `x` occurs here
[01:36:19]   |          borrow of `x` occurs here
[01:36:19] 
[01:36:19] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0510 (line 8603)' panicked at 'Some expected error codes were not found: ["E0510"]', src/librustdoc/test.rs:359:9
[01:36:19] 
[01:36:19] 
[01:36:19] failures:
[01:36:19]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0510 (line 8603)
---
[01:36:19] 
[01:36:19] 
[01:36:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:19] Build completed unsuccessfully in 0:36:56
[01:36:19] make: *** [check] Error 1
[01:36:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16a09c46
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 21:55:57 UTC 2019
---
34288 ./obj/build/x86_64-unkntravis_time:end:265ba1ba:start=1549230959344753570,finish=1549230959395488341,duration=50734771
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0453e2a7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f20af10
$ dmesg | grep -i kill
