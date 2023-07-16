plain
[00:48:51] ...........................................................................i........................
[00:48:56] ....................................................................................................
[00:49:02] ....................................................................................................
[00:49:08] ....................................................................................................
[00:49:13] ........i.................iiiiiiiii...................................................
[00:49:13] 
[00:49:13] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:50:06] ...........................................................................i........................
[00:50:11] ....................................................................................................
[00:50:16] ....................................................................................................
[00:50:22] ....................................................................................................
[00:50:26] ........i.................iiiiiiiii...................................................
[00:50:26] 
[00:50:26]  finished in 73.493
[00:50:26] travis_fold:end:test_ui_nll

---
travis_time:start:test_run-fail
Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:27] 
[01:02:27] running 142 tests
[01:02:38] .........................................................................FF.F.......................
[01:02:43]    |
[01:02:43] 16 |     C[10]
[01:02:43]    |     ^^^^^
[01:02:43]    |
[01:02:43]    |
[01:02:43]    = note: #[deny(const_err)] on by default
[01:02:43] error: aborting due to previous error
[01:02:43] 
[01:02:43] 
[01:02:43] ------------------------------------------
[01:02:43] ------------------------------------------
[01:02:43] 
[01:02:43] thread '[run-fail] run-fail/mir_indexing_oob_1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:02:43] 
[01:02:43] ---- [run-fail] run-fail/mir_indexing_oob_2.rs stdout ----
[01:02:43] 
[01:02:43] error: compilation failed!
[01:02:43] status: exit code: 101
[01:02:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/mir_indexing_oob_2.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_2/auxiliary"
[01:02:43] ------------------------------------------
[01:02:43] 
[01:02:43] ------------------------------------------
[01:02:43] stderr:
[01:02:43] stderr:
[01:02:43] ------------------------------------------
[01:02:43] error: index out of bounds: the len is 5 but the index is 10
[01:02:43]   --> /checkout/src/test/run-fail/mir_indexing_oob_2.rs:16:5
[01:02:43] 16 |     C[10]
[01:02:43]    |     ^^^^^
[01:02:43]    |
[01:02:43]    |
[01:02:43]    = note: #[deny(const_err)] on by default
[01:02:43] error: abor04 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
34372 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
33884 ./src/llvm-emscripten/lib/Target
