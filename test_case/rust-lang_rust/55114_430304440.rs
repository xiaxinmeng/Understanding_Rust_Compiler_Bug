plain
[00:45:48] .................................................................................................... 2200/4601
[00:45:52] ...................i................................................................................ 2300/4601
[00:45:55] .................................................................................................... 2400/4601
[00:45:58] .................................................................................................... 2500/4601
[00:46:02] ................................iiiiiiiii........................................................... 2600/4601
[00:46:07] .................................................................................................... 2800/4601
[00:46:11] .................................................................................................... 2900/4601
[00:46:14] ......................................................i............................................. 3000/4601
[00:46:16] .................................................................................................... 3100/4601
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:00] 
[00:58:00] running 111 tests
[00:58:02] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[00:58:03] ..iiii.....
[00:58:03] 
[00:58:03]  finished in 3.147
[00:58:03] travis_fold:end:test_codegen

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:00] 
[00:59:00] running 97 tests
[01:00:53] .........F.....................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:02:42] .........................F........
[01:02:42] 
[01:02:42] ---- [run-pass] run-pass-fulldeps/dropck_tarena_sound_drop.rs stdout ----
[01:02:42] 
[01:02:42] 
[01:02:42] error: test compilation failed although it shouldn't!
[01:02:42] status: exit code: 1
[01:02:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/dropck_tarena_sound_drop.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/dropck_tarena_sound_drop/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/dropck_tarena_sound_drop/auxiliary"
[01:02:42] ------------------------------------------
[01:02:42] 
[01:02:42] ------------------------------------------
[01:02:42] stderr:
[01:02:42] stderr:
[01:02:42] ------------------------------------------
[01:02:42] {"message":"no function or associated item named `new` found for type `arena::TypedArena<_>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n