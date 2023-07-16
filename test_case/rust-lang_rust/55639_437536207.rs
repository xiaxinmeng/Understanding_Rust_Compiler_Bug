plain
travis_time:end:340e1718:start=1541803341382906177,finish=1541803425480490026,duration=84097583849
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:24] .................................................................................................... 100/5000
[00:53:27] .................................................................................................... 200/5000
[00:53:30] ........................................................................ii...................ii..... 300/5000
[00:53:33] ...........................................................................................iii...... 400/5000
[00:53:35] ..iiiiiiii.iii...........................iii...........................................i...........i 500/5000
[00:53:43] .................................................................................................... 700/5000
[00:53:49] .....................................................................i...........i.................. 800/5000
[00:53:52] ........................................................................................iiiii....... 900/5000
[00:53:56] ............iiiiii.................................................................................. 1000/5000
---
[00:54:32] .................................................................................................... 2200/5000
[00:54:36] .................................................................................................... 2300/5000
[00:54:40] .................................................................................................... 2400/5000
[00:54:44] .................................................................................................... 2500/5000
[00:54:47] .................................................................iiiiiiiii.......................... 2600/5000
[00:54:54] .............................ii..................................................................... 2800/5000
[00:54:57] .................................................................................................... 2900/5000
[00:55:01] .................................................................................................... 3000/5000
[00:55:04] ........................i........................................................................... 3100/5000
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:23] 
[01:09:23] running 115 tests
[01:09:26] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:09:27] .i....iiii.....
[01:09:27] 
[01:09:27]  finished in 3.558
[01:09:27] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:41] 
[01:09:41] running 118 tests
[01:10:04] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:10:08] ......iii.i.....ii
[01:10:08] 
[01:10:08]  finished in 26.711
[01:10:08] travis_fold:end:test_debuginfo

---
[01:42:20] 
[01:42:20] failures:
[01:42:20] 
[01:42:20] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0162 (line 2802) stdout ----
[01:42:20] warning: irrefutable if-let pattern
[01:42:20]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2807:1
[01:42:20]    |
[01:42:20] 7  | / if let Irrefutable(x) = irr {
[01:42:20] 8  | |     // This body will always be executed.
[01:42:20] 9  | |     // ...
[01:42:20]    | |_^
[01:42:20]    |
[01:42:20]    |
[01:42:20]    = note: #[warn(irrefutable_let_patterns)] on by default
[01:42:20] 
[01:42:20] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0162 (line 2802)' panicked at 'test compiled while it wasn't supposed to', librustdoc/test.rs:313:13
[01:42:20] 
[01:42:20] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0165 (line 2856) stdout ----
[01:42:20] warning: irrefutable while-let pattern
[01:42:20]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2861:1
[01:42:20]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2861:1
[01:42:20]   |
[01:42:20] 7 | / while let Irrefutable(x) = irr {
[01:42:20] 8 | |     // ...
[01:42:20]   | |_^
[01:42:20]   |
[01:42:20]   |
[01:42:20]   = note: #[warn(irrefutable_let_patterns)] on by default
[01:42:20] 
[01:42:20] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0165 (line 2856)' panicked at 'test compiled while it wasn't supposed to', librustdoc/test.rs:313:13
[01:42:20] 
[01:42:20] failures:
[01:42:20]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0162 (line 2802)
[01:42:20]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0165 (line 2856)
---
[01:42:20] 
[01:42:20] 
[01:42:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:20] Build completed unsuccessfully in 0:52:48
[01:42:20] make: *** [check] Error 1
[01:42:20] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0eb67535
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Nov 10 00:26:16 UTC 2018
