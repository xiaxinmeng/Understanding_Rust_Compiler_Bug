plain
travis_time:end:0418bbc2:start=1542809233098070872,finish=1542809288130516335,duration=55032445463
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:56:02] .................................................................................................... 100/5043
[00:56:05] .................................................................................................... 200/5043
[00:56:08] .............................ii............................................ii....................ii. 300/5043
[00:56:10] ..............................................................................................iii... 400/5043
[00:56:13] .....iiiiiiii.iii............................iii...........................................i........ 500/5043
[00:56:21] .................................................................................................... 700/5043
[00:56:27] ..................................................................................i...........i..... 800/5043
[00:56:30] .................................................................................................... 900/5043
[00:56:34] .iiiii..................ii.iiii..................................................................... 1000/5043
---
[00:57:10] .................................................................................................... 2200/5043
[00:57:14] .................................................................................................... 2300/5043
[00:57:18] .................................................................................................... 2400/5043
[00:57:22] .................................................................................................... 2500/5043
[00:57:26] .....................................................................................iiiiiiiii...... 2600/5043
[00:57:33] ...................................................ii............................................... 2800/5043
[00:57:36] .................................................................................................... 2900/5043
[00:57:40] .................................................................................................... 3000/5043
[00:57:43] ...............................................i.................................................... 3100/5043
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:14] 
[01:12:14] running 117 tests
[01:12:17] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:12:17] i.i.....iiii.....
[01:12:17] 
[01:12:17]  finished in 3.617
[01:12:17] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:32] 
[01:12:32] running 118 tests
[01:12:56] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:13:00] ......iii.i.....ii
[01:13:00] 
[01:13:00]  finished in 27.617
[01:13:00] travis_fold:end:test_debuginfo

---
[01:47:39] 
[01:47:39] failures:
[01:47:39] 
[01:47:39] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223) stdout ----
[01:47:39] error: reached recursion limit while checking inhabitedness of `Foo`
[01:47:39] 
[01:47:39] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223)' panicked at 'Some expected error codes were not found: ["E0055"]', librustdoc/test.rs:328:9
[01:47:39] 
[01:47:39] 
[01:47:39] failures:
[01:47:39]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0055 (line 1223)
---
[01:47:39] 
[01:47:39] 
[01:47:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:39] Build completed unsuccessfully in 0:55:41
[01:47:39] make: *** [check] Error 1
[01:47:39] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03effc4c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 15:55:55 UTC 2018
