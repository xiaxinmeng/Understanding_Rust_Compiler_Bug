plain
[00:51:12] .................................................................................................... 2000/4317
[00:51:15] ..................................................................i................................. 2100/4317
[00:51:18] .................................................................................................... 2200/4317
[00:51:21] .................................................................................................... 2300/4317
[00:51:24] ...............iiiiiiiii............................................................................ 2400/4317
[00:51:29] .................................................................................................... 2600/4317
[00:51:33] ...................................................................................................i 2700/4317
[00:51:36] .................................................................................................... 2800/4317
[00:51:39] ...........................................................i.i..ii.................................. 2900/4317
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:50] 
[01:04:50] running 106 tests
[01:04:53] i..ii..iii....i...i.........i..iii...........i.....i....ii...i.i.ii..............i...ii..ii.i....iii 100/106
[01:04:53] test result: ok. 76 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:04:53] 
[01:04:53]  finished in 3.355
[01:04:53] travis_fold:end:test_codegen
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:55] 
[01:04:55] running 92 tests
[01:05:07] .......................................F....................................................
[01:05:07] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[01:05:07] 
[01:05:07] ---- [incremental] incremental/hashes/trait_defs.rs stdout ----
[01:05:07] 
[01:05:07] 
[01:05:07] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:05:07] status: exit code: 1
[01:05:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
[01:05:07] ------------------------------------------
[01:05:07] 
[01:05:07] ------------------------------------------
[01:05:07] stderr:
[01:05:07] stderr:
[01:05:07] ------------------------------------------
[01:05:07] {"message":"`Hir(TraitChangeModeSelfOwnToMut::method)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/trait_defs.rs","byte_start":7313,"byte_end":7335,"line_start":277,"line_end":277,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    fn method(mut self) {}","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Hir(TraitChangeModeSelfOwnToMut::method)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/trait_defs.rs:277:5\n   |\nLL |     fn method(mut self) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"re
