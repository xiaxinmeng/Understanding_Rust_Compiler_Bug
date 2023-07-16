plain
travis_time:end:279deb40:start=1541449625166791444,finish=1541449690881726772,duration=65714935328
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:35] .................................................................................................... 100/4990
[00:49:38] .................................................................................................... 200/4990
[00:49:41] ...........................................................................................ii....... 300/4990
[00:49:44] .........................................................................................iii........ 400/4990
[00:49:47] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4990
[00:49:54] .................................................................................................... 700/4990
[00:50:01] ..................................................................i...........i..................... 800/4990
[00:50:04] .....................................................................................iiiii.......... 900/4990
[00:50:07] .................................................................................................... 1000/4990
---
[00:50:42] .................................................................................................... 2200/4990
[00:50:46] .................................................................................................... 2300/4990
[00:50:50] .................................................................................................... 2400/4990
[00:50:54] .................................................................................................... 2500/4990
[00:50:57] ......................................................................iiiiiiiii..................... 2600/4990
[00:51:04] .....................ii............................................................................. 2800/4990
[00:51:07] .................................................................................................... 2900/4990
[00:51:10] .................................................................................................... 3000/4990
[00:51:13] ................i................................................................................... 3100/4990
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:33] 
[01:04:33] running 115 tests
[01:04:36] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:04:36] .i....iiii.....
[01:04:36] 
[01:04:36]  finished in 3.403
[01:04:36] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:50] 
[01:04:50] running 118 tests
[01:05:14] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:18] ......iii.i.....ii
[01:05:18] 
[01:05:18]  finished in 27.882
[01:05:18] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:55] 
[01:05:55] running 97 tests
[01:07:54] ......................................F......................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:10:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:10:30] failures:
[01:10:30] 
[01:10:30] ---- [run-pass] run-pass-fulldeps/mbe_matching_test_macro.rs stdout ----
[01:10:30] 
[01:10:30] 
[01:10:30] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs" failed to compile: 
[01:10:30] status: exit code: 1
[01:10:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mbe_matching_test_macro/auxiliary" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/mbe_matching_test_macro/auxiliary"
[01:10:30] ------------------------------------------
[01:10:30] 
[01:10:30] ------------------------------------------
[01:10:30] stderr:
[01:10:30] stderr:
[01:10:30] ------------------------------------------
[01:10:30] {"message":"unused import: `quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+)`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs","byte_start":1334,"byte_end":1384,"line_start":38,"line_end":38,"column_start":23,"column_end":73,"is_primary":true,"text":[{"text":"    let mbe_matcher = quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+);","highlight_start":23,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+)`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:38:23\n   |\nLL |     let mbe_matcher = quote_tokens!(cx, $$matched:expr, $$($$pat:pat)|+);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:10:30] {"message":"this pattern has 2 fields, but the corresponding tuple variant has 3 fields","code":{"code":"E0023","explanation":"\nA pattern used to match against an enum variant must provide a sub-pattern for\neach field of the enum variant. This error indicates that a pattern attempted to\nextract an incorrect number of fields from a variant.\n\n