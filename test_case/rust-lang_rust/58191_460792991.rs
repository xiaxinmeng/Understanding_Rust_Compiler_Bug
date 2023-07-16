plain
travis_time:end:01c86527:start=1549393256486997182,finish=1549393259038598355,duration=2551601173
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
[01:07:50] 
[01:07:50] running 119 tests
[01:08:15] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:08:18] i......iii.i.....ii
[01:08:18] 
[01:08:18]  finished in 28.268
[01:08:18] travis_fold:end:test_debuginfo

---
[01:33:04] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0670 (line 10979) stdout ----
[01:33:04] error[E0670]: const parameters cannot depend on type parameters
[01:33:04]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10980:25
[01:33:04]   |
[01:33:04] 3 | fn const_id<T, const N: T>() -> T {
[01:33:04]   |                         ^ const parameter depends on type parameter
[01:33:04] error[E0658]: const generics are unstable (see issue #44580)
[01:33:04]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10980:22
[01:33:04]   |
[01:33:04]   |
[01:33:04] 3 | fn const_id<T, const N: T>() -> T {
[01:33:04]   |
[01:33:04]   = help: add #![feature(const_generics)] to the crate attributes to enable
[01:33:04] 
[01:33:04] 
[01:33:04] error: const generics in any position are currently unsupported
[01:33:04]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10980:22
[01:33:04]   |
[01:33:04] 3 | fn const_id<T, const N: T>() -> T {
[01:33:04] 
[01:33:04] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0670 (line 10979)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:33:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:33:04] 
---
[01:33:04] 
[01:33:04] 
[01:33:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:04] Build completed unsuccessfully in 0:36:23
[01:33:04] Makefile:48: recipe for target 'check' failed
[01:33:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03c1cd33
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 20:34:15 UTC 2019
---
travis_time:end:168becb8:start=1549398857504507500,finish=1549398857558101526,duration=53594026
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:097adbf4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11326d9a
$ dmesg | grep -i kill
