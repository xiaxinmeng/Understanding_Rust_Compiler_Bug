plain
[00:51:23] .................................................................................................... 2000/4316
[00:51:26] ..................................................................i................................. 2100/4316
[00:51:29] .................................................................................................... 2200/4316
[00:51:32] .................................................................................................... 2300/4316
[00:51:35] ...............iiiiiiiii............................................................................ 2400/4316
[00:51:40] .................................................................................................... 2600/4316
[00:51:44] ...................................................................................................i 2700/4316
[00:51:47] .................................................................................................... 2800/4316
[00:51:50] ...........................................................i.i..ii.................................. 2900/4316
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:46] 
[01:05:46] running 107 tests
[01:05:50] i..ii...iii....i...i.........i..iii...........i.....i....ii...i.i.ii..............i....ii.ii.i....ii 100/107
[01:05:50] test result: ok. 77 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:50] 
[01:05:50]  finished in 3.444
[01:05:50] travis_fold:end:test_codegen
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:16] 
[01:06:16] running 42 tests
[01:06:51] ............................F......F......
[01:06:51] 
[01:06:51] ---- [ui] ui-fulldeps/proc-macro/multispan.rs stdout ----
[01:06:51] 
[01:06:51] 
[01:06:51] error: auxiliary build of "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/multispan.rs" failed to compile: 
[01:06:51] status: exit code: 1
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/multispan.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/multispan/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/multispan/auxiliary"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] {"message":"use of unstable library feature 'proc_macro_def_site' (see issue #54724)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n