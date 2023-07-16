plain
travis_time:end:0a4903a9:start=1541685757074815692,finish=1541685960292349595,duration=203217533903
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:07] .................................................................................................... 100/4999
[00:53:11] .................................................................................................... 200/4999
[00:53:14] ........................................................................ii...................ii..... 300/4999
[00:53:17] ...........................................................................................iii...... 400/4999
[00:53:20] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4999
[00:53:28] .................................................................................................... 700/4999
[00:53:34] .....................................................................i...........i.................. 800/4999
[00:53:37] ........................................................................................iiiii....... 900/4999
[00:53:41] ...........ii.iiii.................................................................................. 1000/4999
---
[00:54:20] .................................................................................................... 2200/4999
[00:54:25] .................................................................................................... 2300/4999
[00:54:29] .................................................................................................... 2400/4999
[00:54:33] .................................................................................................... 2500/4999
[00:54:37] ...................................................................iiiiiiiii........................ 2600/4999
[00:54:45] ...............................ii................................................................... 2800/4999
[00:54:48] .................................................................................................... 2900/4999
[00:54:52] .................................................................................................... 3000/4999
[00:54:55] ..........................i......................................................................... 3100/4999
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:16] 
[01:09:16] running 115 tests
[01:09:19] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:09:19] .i....iiii.....
[01:09:19] 
[01:09:19]  finished in 3.593
[01:09:19] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:34] 
[01:09:34] running 118 tests
[01:09:59] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:10:03] ......iii.i.....ii
[01:10:03] 
[01:10:03]  finished in 29.168
[01:10:03] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:42] 
[01:10:42] running 97 tests
[01:12:44] ......................................F.....................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:15:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:15:26] failures:
[01:15:26] 
[01:15:26] ---- [run-pass] run-pass-fulldeps/mbe_matching_test_macro.rs stdout ----
[01:15:26] ---- [run-pass] run-pass-fulldeps/mbe_matching_test_macro.rs stdout ----
[01:15:26] 
[01:15:26] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs" failed to compile: 
[01:15:26] status: exit code: 1
[01:15:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mbe_matching_test_macro/auxiliary" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mbe_matching_test_macro/auxiliary"
[01:15:26] ------------------------------------------
[01:15:26] 
[01:15:26] ------------------------------------------
[01:15:26] stderr:
[01:15:26] stderr:
[01:15:26] ------------------------------------------
[01:15:26] {"message":"unused import: `quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+)`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs","byte_start":1334,"byte_end":1384,"line_start":38,"line_end":38,"column_start":23,"column_end":73,"is_primary":true,"text":[{"text":"    let mbe_matcher = quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+);","highlight_start":23,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+)`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:38:23\n   |\nLL |     let mbe_matcher = quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:15:26] {"message":"no function or associated item named `new` found for type `syntax::ast::NodeId` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n