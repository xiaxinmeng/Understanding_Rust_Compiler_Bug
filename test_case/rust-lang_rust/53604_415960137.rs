plain
[00:48:52] ....................................................................................................
[00:48:55] ....................................................................................................
[00:48:58] ................i...................................................................................
[00:49:01] ....................................................................................................
[00:49:04] .................................................................iiiiiiiii..........................
[00:49:09] ....................................................................................................
[00:49:13] ....................................................................................................
[00:49:16] ..............................................i.....................................................
[00:49:19] ................................................................................................i.i.
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:59] 
[00:55:59] running 46 tests
2i128) -> bb11;
[00:56:18] ... (elided)
[00:56:18]     _6 = move (_10.0: i128);
[00:56:18] ... (elided)
[00:56:18]     _11 = const compiler_builtins::int::mul::rust_i128_mulo(move _6, const 3i128) -> bb12;
[00:56:18] ... (elided)
[00:56:18]     _5 = move (_11.0: i128);
[00:56:18] ... (elided)
[00:56:18]     _12 = Eq(const 4i128, const 0i128);
[00:56:18]     assert(!move _12, "attempt to divide by zero") -> bb4;
[00:56:18] ... (elided)
[00:56:18]     _13 = Eq(const 4i128, const -1i128);
[00:56:18]     _14 = Eq(_5, const -170141183460469231731687303715884105728i128);
[00:56:18]     _15 = BitAnd(move _13, move _14);
[00:56:18]     assert(!move _15, "attempt to divide with overflow") -> bb5;
[00:56:18] ... (elided)
[00:56:18]     _4 = const compiler_builtins::int::sdiv::rust_i128_div(move _5, const 4i128) -> bb13;
[00:56:18] ... (elided)
[00:56:18]     _17 = Eq(const 5i128, const -1i128);
[00:56:18]     _18 = Eq(_4, const -170141183460469231731687303715884105728i128);
[00:56:18]     _19 = BitAnd(move _17, move _18);
[00:56:18]     assert(!move _19, "attempt to calculate the remainder with overflow") -> bb7;
[00:56:18] ... (elided)
[00:56:18]     _3 = const compiler_builtins::int::sdiv::rust_i128_rem(move _4, const 5i128) -> bb15;
[00:56:18] ... (elided)
[00:56:18]     _2 = move (_20.0: i128);
[00:56:18] ... (elided)
[00:56:18]     _23 = const 7i32 as u128 (Misc);
[00:56:18]     _21 = const compiler_builtins::int::shift::rust_i128_shro(move _2, move _23) -> bb16;
[00:56:18] ... (elided)
[00:56:18]     _0 = move (_21.0: i128);
[00:56:18] ... (elided)
[00:56:18]     assert(!move (_9.1: bool), "attempt:18] 
[00:56:18]     [mir-opt] mir-opt/lower_128bit_test.rs
[00:56:18] 
[00:56:18] test result: FAILED. 45 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:18] 
[00:56:18] 
[00:56:18] 
[00:56:18] 
[00:56:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:18] 
[00:56:18] 
[00:56:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:18] Build completed unsuccessful193fx8
