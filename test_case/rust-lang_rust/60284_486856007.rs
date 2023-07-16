plain
travis_time:end:1ccfe540:start=1556223391941397866,finish=1556223392717053800,duration=775655934
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
[01:24:24] 
[01:24:24] running 9 tests
[01:24:24] iiiiiiiii
[01:24:24] 
[01:24:24]  finished in 0.149
[01:24:24] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:40] 
[01:24:40] running 121 tests
[01:25:04] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:25:08] i.i......iii.i.....ii
[01:25:08] 
[01:25:08]  finished in 28.186
[01:25:08] travis_fold:end:test_debuginfo

---
[01:53:38] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0671 (line 11021) stdout ----
[01:53:38] error[E0671]: const parameters cannot depend on type parameters
[01:53:38]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11022:25
[01:53:38]   |
[01:53:38] 3 | fn const_id<T, const N: T>() -> T {
[01:53:38]   |                         ^ const parameter depends on type parameter
[01:53:38] error[E0658]: const generics are unstable
[01:53:38]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11022:22
[01:53:38]   |
[01:53:38]   |
[01:53:38] 3 | fn const_id<T, const N: T>() -> T {
[01:53:38]   |
[01:53:38]   = note: for more information, see https://github.com/rust-lang/rust/issues/44580
[01:53:38]   = help: add #![feature(const_generics)] to the crate attributes to enable
[01:53:38] 
---
[01:53:38] 
[01:53:38] 
[01:53:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:53:38] Build completed unsuccessfully in 0:41:19
[01:53:38] make: *** [check] Error 1
[01:53:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002b9924
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 22:10:23 UTC 2019
