plain
travis_time:end:03744f28:start=1543813647748471347,finish=1543813708883239185,duration=61134767838
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:51] 
[00:58:51] running 119 tests
[00:58:54] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:58:55] .ii.i.....iii.i....
[00:58:55] 
[00:58:55]  finished in 3.636
[00:58:55] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:11] 
[00:59:11] running 118 tests
[00:59:36] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:59:40] ......iii.i.....ii
[00:59:40] 
[00:59:40]  finished in 29.669
[00:59:40] travis_fold:end:test_debuginfo

---
[01:27:02] 
[01:27:02] failures:
[01:27:02] 
[01:27:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0162 (line 2802) stdout ----
[01:27:02] warning: irrefutable if-let pattern
[01:27:02]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2807:1
[01:27:02]    |
[01:27:02] 7  | / if let Irrefutable(x) = irr {
[01:27:02] 8  | |     // This body will always be executed.
[01:27:02] 9  | |     // ...
[01:27:02]    | |_^
[01:27:02]    |
[01:27:02]    |
[01:27:02]    = note: #[warn(irrefutable_let_patterns)] on by default
[01:27:02] 
[01:27:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0162 (line 2802)' panicked at 'test compiled while it wasn't supposed to', src/librustdoc/test.rs:313:13
[01:27:02] 
[01:27:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0165 (line 2856) stdout ----
[01:27:02] warning: irrefutable while-let pattern
[01:27:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2861:1
[01:27:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2861:1
[01:27:02]   |
[01:27:02] 7 | / while let Irrefutable(x) = irr {
[01:27:02] 8 | |     // ...
[01:27:02]   | |_^
[01:27:02]   |
[01:27:02]   |
[01:27:02]   = note: #[warn(irrefutable_let_patterns)] on by default
0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
58684 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps
56896 ./src/llvm/test/MC
56108 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build
