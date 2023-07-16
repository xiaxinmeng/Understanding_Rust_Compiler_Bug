plain
travis_time:end:27ff0495:start=1552585191980436648,finish=1552585271763140515,duration=79782703867
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:50] 
[01:22:50] running 120 tests
[01:23:15] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:23:20] .i......iii.i.....ii
[01:23:20] 
[01:23:20]  finished in 30.632
[01:23:20] travis_fold:end:test_debuginfo

---
[01:51:10] 
[01:51:10] failures:
[01:51:10] 
[01:51:10] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0725 (line 11586) stdout ----
[01:51:10] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0725 (line 11586)' panicked at 'test compiled while it wasn't supposed to', src/librustdoc/test.rs:300:13
[01:51:10] 
[01:51:10] 
[01:51:10] failures:
[01:51:10]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0725 (line 11586)
---
[01:51:10] 
[01:51:10] 
[01:51:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:51:10] Build completed unsuccessfully in 0:40:24
[01:51:10] make: *** [check] Error 1
[01:51:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0333c61b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 14 19:32:31 UTC 2019
