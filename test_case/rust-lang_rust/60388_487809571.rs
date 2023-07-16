plain
travis_time:end:2d7196b8:start=1556587498832707828,finish=1556587499655449894,duration=822742066
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
[01:22:42] 
[01:22:42] running 9 tests
[01:22:42] iiiiiiiii
[01:22:42] 
[01:22:42]  finished in 0.169
[01:22:42] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:58] 
[01:22:58] running 121 tests
[01:23:24] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:23:29] i.i......iii.i.....ii
[01:23:29] 
[01:23:29]  finished in 30.873
[01:23:29] travis_fold:end:test_debuginfo

---
[01:46:57] 
[01:46:57] failures:
[01:46:57] 
[01:46:57] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0106 (line 2020) stdout ----
[01:46:57] error[E0726]: implicit elided lifetime not allowed here
[01:46:57]   |
[01:46:57] 7 | impl Foo2 {}
[01:46:57]   |      ^^^^
[01:46:57]   |
[01:46:57]   |
[01:46:57]   = help: consider using an explicit elided lifetime: `'_`
[01:46:57] error: aborting due to previous error
[01:46:57] 
[01:46:57] 
[01:46:57] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0106 (line 2020)' panicked at 'Some expected error codes were not found: ["E0106"]', src/librustdoc/test.rs:315:9
[01:46:57] 
[01:46:57] 
[01:46:57] failures:
[01:46:57]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0106 (line 2020)
---
[01:46:57] 
[01:46:57] 
[01:46:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:57] Build completed unsuccessfully in 0:36:22
[01:46:57] make: *** [check] Error 1
[01:46:57] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:038be480
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 03:12:08 UTC 2019
