plain
[00:48:29] .................................................................................................... 2200/4601
[00:48:33] ...................i................................................................................ 2300/4601
[00:48:36] .................................................................................................... 2400/4601
[00:48:40] .................................................................................................... 2500/4601
[00:48:44] ................................iiiiiiiii........................................................... 2600/4601
[00:48:49] .................................................................................................... 2800/4601
[00:48:53] .................................................................................................... 2900/4601
[00:48:56] ......................................................i............................................. 3000/4601
[00:48:59] .................................................................................................... 3100/4601
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:06] 
[01:01:06] running 111 tests
[01:01:08] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:01:09] ..iiii.....
[01:01:09] 
[01:01:09]  finished in 3.279
[01:01:09] travis_fold:end:test_codegen

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:06] 
[01:02:06] running 97 tests
[01:04:01] ..............................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:05:51] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[01:05:51] .........................F.........
[01:05:51] 
[01:05:51] ---- [run-pass] run-pass-fulldeps/regions-mock-tcx.rs stdout ----
[01:05:51] 
[01:05:51] 
[01:05:51] error: test compilation failed although it shouldn't!
[01:05:51] status: exit code: 1
[01:05:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/regions-mock-tcx.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/regions-mock-tcx/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/regions-mock-tcx/auxiliary"
[01:05:51] ------------------------------------------
[01:05:51] 
[01:05:51] ------------------------------------------
[01:05:51] stderr:
[01:05:51] stderr:
[01:05:51] ------------------------------------------
[01:05:51] {"message":"no function or associated item named `new` found for type `arena::TypedArena<_>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n