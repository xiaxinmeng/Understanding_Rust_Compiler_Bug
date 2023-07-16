plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:46:54] 
[00:46:54] running 6788 tests
[00:46:57] .....................................F.........................................F....................
[00:47:00] ............................................F........................F.............F................
[00:47:05] ....................................................................................................
[00:47:05] ....................................................................................................
[00:47:08] ...................................F...............................................................F
[00:47:12] .F.F...........F.i.....F.....F......................................................................
[00:47:17] ....................................................................................................
[00:47:21] F......................F..i...........i......................F......................................
[00:47:24] ...............................F.......F...Fiiiii...............................FF..F.F.............
[00:47:27] .......................................................................................F............
[00:47:29] ....................................................................................................
[00:47:31] ..............................................................................................F.....
[00:47:34] ...............F...F......F...................F.....................................................
[00:47:36] ....................................................................................................
[00:47:38] ...................F....F.F........i.F......................F.....................F.F.F.FFF.F.......
[00:47:41] ..i..............................................................F...................FF.............
[00:47:45] ........................................F..F............F..................................F........
[00:47:47] ..F.............................F..............F.......................F.................i..........
[00:47:50] ....................................................................................................
[00:47:53] ......................................................................F...F......F.......F..........
[00:47:56] ....................................................................................................
[00:47:59] ...........................................................F.....................................F..
[00:48:02] .............................F...........F.......F........................F......................F..
[00:48:05] .........................................................F........................F.....F...........
[00:48:08] ..............F..............................F......................................................
[00:48:14] .................................................................................i..................
[00:48:14] .................................................................................i..................
[00:48:17] ........................................................................................F.......F...
[00:48:20] .F.......F.........FF...............i.i..ii...........................................F.............
[00:48:24] ....................................................................................................
[00:48:27] ..............................................F.i........................................F.F........
[00:48:30] ....................................................................................................
[00:48:33] ....F.............................................................F........FF.......................
[00:48:36] .............F...........................F...................................................F......
[00:48:39] ..........................F...............................F....................i....................
[00:48:53] ....................................................................................................
[00:49:01] ....................................................................................................
[00:49:08] ....................................................................................................
[00:49:17] ....................................................................................................
---
[00:52:29] ....................................................................................................
[00:52:58] ....................................................................................................
[00:53:06] ....................................................................................................
[00:53:16] ....................................................................................................
[00:53:19] .............................................................................................F......
[00:53:22] ...................F.................F......................................F..F....................
[00:53:26] ..F.....................................F.FF...........................F.i....FF....................
[00:53:29] .........F......................................................................F.......F...........
[00:53:31] ........F.F..F.....F....F......................F....................................................
[00:53:34] ........................................F..............................................F............
/src/test/ui/associated-types/associated-types-eq-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] 
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/associated-types/associated-types-eq-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/associated-types/associated-types-eq-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/bad/-------
[00:53:36] 
[00:53:36] thread '[ui] ui/bad/bad-const-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/binop/binop-logic-int.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-logic-int.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-logic-int/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-logic-int/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] 
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
---
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/call-fn-never-arg-wrong-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/call-fn-never-arg-wrong-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/closures/closure-no-fn-1.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-no-fn-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-no-fn-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-no-fn-1/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] 
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/closures/closure-no-fn-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/closures/closure-no-fn-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/closures/closure-no-fn-2.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-no-fn-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-no-fn-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-help-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-reform-bad/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-reform-bad/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] 
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/closures/closure-reform-bad.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/closures/closure-reform-bad.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/codemap_tests/tab.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 1r unexpectedly panicked. this is a bug.
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/codemap_tests/tab.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/codemap_tests/tab.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/coercion/coerce-mut.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-mut/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs-----------------------
[00:53:36] ------------------------------------------
[00:53:36] ------------------------------------------
[00:53:36] thread 'main' panicked at 'attempt to subtract with overflow', librustc_typeck/check/demand.rs:428:38
[00:53:36] 
[00:53:36] error: internal compiler error: unexpected panic
[00:53:36] 
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/coercion/coerce-to-bang.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/coercion/coerce-to-bang.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/consts/const-integer-bool-ops.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-integer-bool-ops.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-integer-bool-ops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" ynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conversion-methods/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conversion-methods/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n