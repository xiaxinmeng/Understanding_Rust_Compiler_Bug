plain
travis_time:end:13195c80:start=1556125954504643088,finish=1556126046443230864,duration=91938587776
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:41] 
[01:17:41] running 9 tests
[01:17:41] iiiiiiiii
[01:17:41] 
[01:17:41]  finished in 0.148
[01:17:41] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:56] 
[01:17:56] running 121 tests
[01:18:22] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:26] i.i......iii.i.....ii
[01:18:26] 
[01:18:26]  finished in 29.641
[01:18:26] travis_fold:end:test_debuginfo

---
[01:45:27] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11347) stdout ----
[01:45:27] error[E0578]: cannot find module `foo` in module `crate`
[01:45:27]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11349:19
[01:45:27]   |
[01:45:27] 4 |     pub(in crate::foo) struct Bar {
[01:45:27] 
[01:45:27] error: aborting due to previous error
[01:45:27] 
[01:45:27] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11347)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:45:27] 
[01:45:27] 
[01:45:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:27] Build completed unsuccessfully in 0:39:14
[01:45:27] Makefile:48: recipe for target 'check' failed
[01:45:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15eaf814
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 18:59:43 UTC 2019
---
travis_time:end:0b55f315:start=1556132384932917522,finish=1556132384985185401,duration=52267879
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25b9525f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02eaeca8
$ dmesg | grep -i kill
