plain
[00:52:23] ............................................................................ii......................
[00:53:13] ........................................i....................................................i.ii...
[00:53:20] ................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:53:56] ....................................................................................
[00:54:17] .iiiiiii............................................................................................
[00:54:57] ....................................................................................................
[00:55:15] .........................................................................
[00:55:15] test result: ok. 2954 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[00:55:15] 
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:35] 
[00:58:35] running 50 tests
verflow") -> bb1;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_10.1: bool), "attempt to subtract with overflow") -> bb2;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_11.1: bool), "attempt to multiply with overflow") -> bb3;
[00:58:57] ... (elided)
[00:58:57]     _16 = Eq(const 5i128, const 0i128);
[00:58:57]     assert(!move _16, "attempt to calculate the remainder with a divisor of zero") -> bb6;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_20.1: bool), "attempt to shift left with overflow") -> bb8;
[00:58:57] ... (elided)
[00:58:57]     _22 = const 6i32 as u128 (Misc);
[00:58:57]     _20 = const compiler_builtins::int::shift::rust_i128_shlo(move _3, move _22) -> bb14;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_21.1: bool), "attempt to shift right with overflow") -> bb9;
[00:58:57] Actual:
[00:58:57] fn const_signed(_1: i128) -> i128{
[00:58:57]     let mut _0: i128;
[00:58:57]     let mut _2: i128;
[00:58:57]     let mut _3: i128;
[00:58:57]     let mut _4: i128;
[00:58:57]     let mut _5: i128;
[00:58:57]     let mut _6: i128;
[00:58:57]     let mut _7: i128;
[00:58:57]     let mut _8: i128;
[00:58:57]     let mut _9: (i128, bool);
[00:58:57]     let mut _10: (i128, bool);
[00:58:57]     let mut _11: (i128, bool);
[00:58:57]     let mut _12: bool;
[00:58:57]     let mut _13: bool;
[00:58:57]     let mut _14: bool;
[00:58:57]     let mut _15: bool;
[00:58:57]     let mut _16: bool;
[00:58:57]     let mut _17: bool;
[00:58:57]     let mut _18: bool;
[00:58:57]     let mut _19: bool;
[00:58:57]     let mut _20: (i128, bool);
[00:58:57]     let mut _21: (i128, bool);
[00:58:57]     let mut _22: u128;
[00:58:57]     let mut _23: u128;
[00:58:57]     bb0: {                              
[00:58:57]         StorageLive(_2);
[00:58:57]         StorageLive(_3);
[00:58:57]         StorageLive(_4);
[00:58:57]         StorageLive(_5);
[00:58:57]         StorageLive(_6);
[00:58:57]         StorageLive(_7);
[00:58:57]         StorageLive(_8);
[00:58:57]         _8 = _1;
[00:58:57]         _9 = const compiler_builtins::int::addsub::rust_i128_addo(move _8, const 1i128) -> bb10;
[00:58:57]     }
[00:58:57]     bb1: {                              
[00:58:57]         _7 = move (_9.0: i128);
[00:58:57]         StorageDead(_8);
[00:58:57]         _10 = const compiler_builtins::int::addsub::rust_i128_subo(move _7, const 2i128) -> bb11;
[00:58:57]     }
[00:58:57]     bb2: {                              
[00:58:57]         _6 = move (_10.0: i128);
[00:58:57]         StorageDead(_7);
[00:58:57]         _11 = const compiler_builtins::int::mul::rust_i128_mulo(move _6, const 3i128) -> bb12;
[00:58:57]     }
[00:58:57]     bb3: {                              
[00:58:57]         _5 = move (_11.0: i128);
[00:58:57]         StorageDead(_6);
[00:58:57]         _12 = Eq(const 4i128, const 0i128);
[00:58:57]         assert(!move _12, attempt to divide by zero) -> bb4;
[00:58:57]     }
[00:58:57]     bb4: {                              
[00:58:57]         _13 = Eq(const 4i128, const -1i128);
[00:58:57]         _14 = Eq(_5, const -170141183460469231731687303715884105728i128);
[00:58:57]         _15 = BitAnd(move _13, move _14);
[00:58:57]         assert(!move _15, attempt                  
[00:58:57]         assert(!move (_11.1: bool), attempt to multiply with overflow) -> bb3;
[00:58:57]     }
[00:58:57]     bb13: {                             
[00:58:57]         StorageDead(_5);
[00:58:57]         _16 = Eq(const 5i128, const 0i128);
[00:58:57]         assert(!move _16, attempt to calculate the remainder with a divisor of zero) -> bb6;
[00:58:57]     }
[00:58:57]     bb14: {                             
[00:58:57]         StorageDead(_22);
[00:58:57]         assert(!move (_20.1: bool), attempt to shift left with overflow) -> bb8;
[00:58:57]     }
[00:58:57]     bb15: {                             
[00:58:57]         StorageDead(_4);
[00:58:57]         StorageLive(_22);
[00:58:57]         _22 = const 6i32 as u128 (Misc);
[00:58:57]         _20 = const compiler_builtins::int::shift::rust_i128_shlo(move _3, move _22) -> bb14;
[00:58:57]     }
[00:58:57]     bb16: {                             
[00:58:57]         StorageDead(_23);
[00:58:57]         assert(!move (_21.1: bool), attempt to shift right with overflow) -> bb9;
[00:58:57]     }
[00:58:57] }', tools/compiletest/src/runtest.rs:2735:13
[00:58:57] 
[00:58:57] ---- [mir-opt] mir-opt/lower_128bit_debug_test.rs stdout ----
[00:58:57] ---- [mir-opt] mir-opt/lower_128bit_debug_test.rs stdout ----
[00:58:57]  thread '[mir-opt] mir-opt/lower_128bit_debug_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:58:57] Current block: None
[00:58:57] Actual Line: "        assert(!move _12, attempt to divide by zero) -> bb4;"
[00:58:57] Expected Line: "    assert(!move _12, \"attempt to divide by zero\") -> bb4;"
[00:58:57] Expected:
[00:58:57] ... (elided)
[00:58:57]     _8 = _1;
[00:58:57]     _9 = const compiler_builtins::int::addsub::rust_i128_addo(move _8, const 1i128) -> bb10;
[00:58:57] ... (elided)
[00:58:57]     _7 = move (_9.0: i128);
[00:58:57] ... (elided)
[00:58:57]     _10 = const compiler_builtins::int::addsub::rust_i128_subo(move _7, const 2i128) -> bb11;
[00:58:57] ... (elided)
[00:58:57]     _6 = move (_10.0: i128);
[00:58:57] ... (elided)
[00:58:57]     _11 = const compiler_builtins::int::mul::rust_i128_mulo(move _6, const 3i128) -> bb12;
[00:58:57] ... (elided)
[00:58:57]     _5 = move (_11.0: i128);
[00:58:57] ... (elided)
[00:58:57]     _12 = Eq(const 4i128, const 0i128);
[00:58:57]     assert(!move _12, "attempt to divide by zero") -> bb4;
[00:58:57] ... (elided)
[00:58:57]     _13 = Eq(const 4i128, const -1i128);
[00:58:57]     _14 = Eq(_5, const -170141183460469231731687303715884105728i128);
[00:58:57]     _15 = BitAnd(move _13, move _14);
[00:58:57]     assert(!move _15, "attempt to divide with overflow") -> bb5;
[00:58:57] ... (elided)
[00:58:57]     _4 = const compiler_builtins::int::sdiv::rust_i128_div(move _5, const 4i128) -> bb13;
[00:58:57] ... (elided)
[00:58:57]     _17 = Eq(const 5i128, const -1i128);
[00:58:57]     _18 = Eq(_4, const -170141183460469231731687303715884105728i128);
[00:58:57]     _19 = BitAnd(move _17, move _18);
[00:58:57]     assert(!move _19, "attempt to calculate the remainder with overflow") -> bb7;
[00:58:57] ... (elided)
[00:58:57]     _3 = const compiler_builtins::int::sdiv::rust_i128_rem(move _4, const 5i128) -> bb15;
[00:58:57] ... (elided)
[00:58:57]     _2 = move (_20.0: i128);
[00:58:57] ... (elided)
[00:58:57]     _23 = const 7i32 as u128 (Misc);
[00:58:57]     _21 = const compiler_builtins::int::shift::rust_i128_shro(move _2, move _23) -> bb16;
[00:58:57] ... (elided)
[00:58:57]     _0 = move (_21.0: i128);
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_9.1: bool), "attempt to add with overflow") -> bb1;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_10.1: bool), "attempt to subtract with overflow") -> bb2;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_11.1: bool), "attempt to multiply with overflow") -> bb3;
[00:58:57] ... (elided)
[00:58:57]     _16 = Eq(const 5i128, const 0i128);
[00:58:57]     assert(!move _16, "attempt to calculate the remainder with a divisor of zero") -> bb6;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_20.1: bool), "attempt to shift left with overflow") -> bb8;
[00:58:57] ... (elided)
[00:58:57]     _22 = const 6i32 as u128 (Misc);
[00:58:57]     _20 = const compiler_builtins::int::shift::rust_i128_shlo(move _3, move _22) -> bb14;
[00:58:57] ... (elided)
[00:58:57]     assert(!move (_21.1: bool), "attempt to shift right with overflow") -> bb9;
[00:58:57] Actual:
[00:58:57] fn const_signed(_1: i128) -> i128{
[00:58:57]     let mut _0: i128;
[00:58:57]     let mut _2: i128;
[00:58:57]     let mut _3: i128;
[00:58:57]     let mut _4: i128;
[00:58:57]     let mut _5: i128;
[00:58:57]     let mut _6: i128;
[00:58:57]     let mut _7: i128;
[00:58:57]     let mut _8: i128;
[00:58:57]     let mut _9: (i128, bool);
[00:58:57]     let mut _10: (i128, bool);
[00:58:57]     let        assert(!move _12, attempt to divide by zero) -> bb4;
[00:58:57]     }
[00:58:57]     bb4: {                              
[00:58:57]         _13 = Eq(const 4i128, const -1i128);
[00:58:57]         _14 = Eq(_5, const -170141183460469231731687303715884105728i128);
[00:58:57]         _15 = BitAnd(move _13, move _14);
[00:58:57]         assert(!move _15, attempt to divide with overflow) -> bb5;
[00:58:57]     }
[00:58:57]     bb5: {                              
[00:58:57]         _4 = const compiler_builtins::int::sdiv::rust_i128_div(move _5, const 4i128) -> bb13;
[00:58:57]     }
[00:58:57]     bb6: {                              
[00:58:57]         _17 = Eq(const 5i128, const -1i128);
[00:58:57]         _18 = Eq(_4, const -170141183460469231731687303715884105728i128);
[00:58:57]         _19 = BitAnd(move _17, move _18);
[00:58:57]         assert(!move _19, attempt to calculate the remainder with overflow) -> bb7;
[00:58:57]     }
[00:58:57]     bb7: {                              
[00:58:57]         _3 = const compiler_builtins::int::sdiv::rust_i128_rem(move _4, const 5i128) -> bb15;
[00:58:57]     }
[00:58:57]     bb8: {                              
[00:58:57]         _2 = move (_20.0: i128);
[00:58:57]         StorageDead(_3);
[00:58:57]         StorageLive(_23);
[00:58:57]         _23 = const 7i32 as u128 (Misc);
[00:58:57]         _21 = const compiler_builtins::int::shift::rust_i128_shro(move _2, move _23) -> bb16;
[00:58:57]     }
[00:58:57]     bb9: {                              
[00:58:57]         _0 = move (_21.0: i128);
[00:58:57]         StorageDead(_2);
[00:58:57]         retur:57]     [mir-opt] mir-opt/lower_128bit_test.rs
[00:58:57] test result: FAILED. 48 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[00:58:57] 
[00:58:57] 
[00:58:57] 
[00:58:57] 
[00:58:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:57] 
[00:58:57] 
[00:58:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:57] Build completed unsuccessfully in 0:17:26
[00:58:57] Build completed unsuccessfully in 0:17:26
[00:58:57] make: ***Sat, 28 Apr 2018 12:36:34 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
