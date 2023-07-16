plain
travis_time:end:041b64c8:start=1556134905771833035,finish=1556135005312301420,duration=99540468385
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
[01:19:55] 
[01:19:55] running 9 tests
[01:19:55] iiiiiiiii
[01:19:55] 
[01:19:55]  finished in 0.154
[01:19:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:11] 
[01:20:11] running 121 tests
[01:20:36] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:20:41] i.i......iii.i.....ii
[01:20:41] 
[01:20:41]  finished in 30.281
[01:20:41] travis_fold:end:test_debuginfo

---
[01:48:16] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11347) stdout ----
[01:48:16] error[E0578]: cannot find module `foo` in module `crate`
[01:48:16]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11349:19
[01:48:16]   |
[01:48:16] 4 |     pub(in crate::foo) struct Bar {
[01:48:16] 
[01:48:16] error: aborting due to previous error
[01:48:16] 
[01:48:16] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11347)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:48:16] 
[01:48:16] 
[01:48:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:16] Build completed unsuccessfully in 0:40:04
[01:48:16] Makefile:48: recipe for target 'check' failed
[01:48:16] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00497e74
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 21:31:51 UTC 2019
