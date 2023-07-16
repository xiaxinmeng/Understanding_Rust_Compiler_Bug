plain
travis_time:end:075f841a:start=1546486231213881997,finish=1546486233360981727,duration=2147099730
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
[01:06:38] 
[01:06:38] running 118 tests
[01:07:01] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:07:05] ......iii.i.....ii
[01:07:05] 
[01:07:05]  finished in 26.883
[01:07:05] travis_fold:end:test_debuginfo

---
[01:33:21] 
[01:33:21] failures:
[01:33:21] 
[01:33:21] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223) stdout ----
[01:33:21] error[E0275]: overflow evaluating the requirement `&Foo: std::marker::Unsize<_>`
[01:33:21]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1234:19
[01:33:21]    |
[01:33:21] 12 |     let ref_foo = &&Foo;
[01:33:21]    |
[01:33:21]    |
[01:33:21]    = help: consider adding a `#![recursion_limit="4"]` attribute to your crate
[01:33:21]    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<*const _>` for `&&Foo`
[01:33:21] 
[01:33:21] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223)' panicked at 'Some expected error codes were not found: ["E0055"]', src/librustdoc/test.rs:321:9
[01:33:21] 
[01:33:21] 
[01:33:21] failures:
[01:33:21]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223)
---
[01:33:21] 
[01:33:21] 
[01:33:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:21] Build completed unsuccessfully in 0:37:30
[01:33:21] Makefile:48: recipe for target 'check' failed
[01:33:21] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1159943d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 05:04:03 UTC 2019
