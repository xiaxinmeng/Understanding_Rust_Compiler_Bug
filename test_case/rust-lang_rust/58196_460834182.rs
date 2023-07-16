plain
travis_time:end:2ccbb0e1:start=1549400790641426547,finish=1549400791511527207,duration=870100660
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
[01:12:25] 
[01:12:25] running 119 tests
[01:12:50] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:12:54] i......iii.i.....ii
[01:12:54] 
[01:12:54]  finished in 29.036
[01:12:54] travis_fold:end:test_debuginfo

---
[01:37:55] 
[01:37:55] failures:
[01:37:55] 
[01:37:55] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0723 (line 11531) stdout ----
[01:37:55] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0723 (line 11531)' panicked at 'test compiled while it wasn't supposed to', src/librustdoc/test.rs:344:13
[01:37:55] 
[01:37:55] 
[01:37:55] failures:
[01:37:55]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0723 (line 11531)
---
[01:37:55] 
[01:37:55] 
[01:37:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:37:55] Build completed unsuccessfully in 0:36:45
[01:37:55] make: *** [check] Error 1
[01:37:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1649cbfa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 22:44:38 UTC 2019
---
travis_time:end:020e8a84:start=1549406679593073818,finish=1549406679642674369,duration=49600551
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:109671ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:165b1336
$ dmesg | grep -i kill
