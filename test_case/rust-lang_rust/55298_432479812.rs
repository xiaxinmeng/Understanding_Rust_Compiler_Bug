plain
[00:50:59] .................................................................................................... 2200/4943
[00:51:04] .................................................................................................... 2300/4943
[00:51:08] .................................................................................................... 2400/4943
[00:51:13] .................................................................................................... 2500/4943
[00:51:17] ................................................iiiiiiiii........................................... 2600/4943
[00:51:21] ..................................................................................................ii 2700/4943
[00:51:29] .................................................................................................... 2900/4943
[00:51:32] ........................................................................................i........... 3000/4943
[00:51:36] .................................................................................................... 3100/4943
[00:51:39] ...............................................i..i.ii.............................................. 3200/4943
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:57] 
[01:04:57] running 111 tests
[01:05:00] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:05:00] ..iiii.....
[01:05:00] 
[01:05:00]  finished in 3.617
[01:05:00] travis_fold:end:test_codegen

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:20] 
[01:06:20] running 97 tests
[01:08:19] ...........................................FFF..........................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:10:05] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:10:05] failures:
[01:10:05] 
[01:10:05] ---- [run-pass] run-pass-fulldeps/plugin-args-1.rs stdout ----
[01:10:05] 
[01:10:05] 
[01:10:05] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile: 
[01:10:05] status: exit code: 1
[01:10:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Zunstable-options" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1/auxiliary"
[01:10:05] ------------------------------------------
[01:10:05] 
[01:10:05] ------------------------------------------
[01:10:05] stderr:
[01:10:05] stderr:
[01:10:05] ------------------------------------------
[01:10:05] {"message":"unused import: `syntax::ptr::P`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs","byte_start":864,"byte_end":878,"line_start":27,"line_end":27,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use syntax::ptr::P;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `syntax::ptr::P`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:27:5\n   |\nLL | use syntax::ptr::P;\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:10:05] {"message":"method `expand` has 4 parameters but the declaration in trait `syntax::ext::base::TTMacroExpander::expand` has 5","code":{"code":"E0050","explanation":"\nThis error indicates that an attempted implementation of a trait method\nhas the wrong number of function parameters.\n\nFor example, the trait below has a method `foo` with two function parameters\n(`&self` and `u8`), but the implementation of `foo` for the type `Bar` omits\nthe `u8` parameter:\n\n