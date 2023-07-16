plain
travis_time:start:test_run-fail
Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:37] 
[01:17:37] running 138 tests
[01:17:50] ............................................................................F....FF........F........
[01:17:55] failures:
[01:17:55] 
[01:17:55] ---- [run-fail] run-fail/overflowing-add.rs stdout ----
[01:17:55]  
[01:17:55]  
[01:17:55] error: compilation failed!
[01:17:55] status: exit code: 101
[01:17:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-add.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-add.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-add.stage2-x86_64-unknown-linux-gnu.aux"
[01:17:55] ------------------------------------------
[01:17:55] 
[01:17:55] ------------------------------------------
[01:17:55] stderr:
[01:17:55] stderr:
[01:17:55] ------------------------------------------
[01:17:55] error: attempt to add with overflow
[01:17:55]    |
[01:17:55]    |
[01:17:55] 15 |     let _x = 200u8 + 200u8 + 200u8;
[01:17:55]    |
[01:17:55]    |
[01:17:55]    = note: #[deny(const_err)] on by default
[01:17:55] error: aborting due to previous error
[01:17:55] 
[01:17:55] 
[01:17:55] ------------------------------------------
[01:17:55] ------------------------------------------
[01:17:55] 
[01:17:55] thread '[run-fail] run-fail/overflowing-add.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:17:55] 
[01:17:55] ---- [run-fail] run-fail/overflowing-mul.rs stdout ----
[01:17:55]  
[01:17:55] error: compilation failed!
[01:17:55] status: exit code: 101
[01:17:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-mul.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-mul.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-mul.stage2-x86_64-unknown-linux-gnu.aux"
[01:17:55] ------------------------------------------
[01:17:55] 
[01:17:55] ------------------------------------------
[01:17:55] stderr:
[01:17:55] stderr:
[01:17:55] ------------------------------------------
[01:17:55] warning: unused variable: `x`
[01:17:55]   --> /checkout/src/test/run-fail/overflowing-mul.rs:15:9
[01:17:55]    |
[01:17:55] 15 |     let x = 200u8 * 4;
[01:17:55]    |         ^ help: consider using `_x` instead
[01:17:55]    = note: #[warn(unused_variables)] on by default
[01:17:55] 
[01:17:55] 
[01:17:55] error: attempt to multiply with overflow
[01:17:55]    |
[01:17:55]    |
[01:17:55] 15 |     let x = 200u8 * 4;
[01:17:55]    |
[01:17:55]    |
[01:17:55]    = note: #[deny(const_err)] on by default
[01:17:55] error: aborting due to previous error
[01:17:55] 
[01:17:55] 
[01:17:55] ------------------------------------------
[01:17:55] ------------------------------------------
[01:17:55] 
[01:17:55] thread '[run-fail] run-fail/overflowing-mul.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:17:55] 
[01:17:55] ---- [run-fail] run-fail/overflowing-neg.rs stdout ----
[01:17:55]  
[01:17:55] error: compilation failed!
[01:17:55] status: exit code: 101
[01:17:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-neg.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-neg.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-neg.stage2-x86_64-unknown-linux-gnu.aux"
[01:17:55] ------------------------------------------
[01:17:55] 
[01:17:55] ------------------------------------------
[01:17:55] stderr:
[01:17:55] stderr:
[01:17:55] ------------------------------------------
[01:17:55] error: attempt to negate with overflow
[01:17:55]    |
[01:17:55]    |
[01:17:55] 15 |     let _x = -std::i8::MIN;
[01:17:55]    |
[01:17:55]    |
[01:17:55]    = note: #[deny(const_err)] on by default
[01:17:55] error: constant evaluation error
[01:17:55]   --> /checkout/src/test/run-fail/overflowing-neg.rs:15:14
[01:17:55]    |
[01:17:55]    |
[01:17:55] 15 |     let _x = -std::i8::MIN;
[01:17:55]    |              ^^^^^^^^^^^^^ attempt to negate with overflow
[01:17:55] error: aborting due to 2 previous errors
[01:17:55] 
[01:17:55] 
[01:17:55] ------------------------------------------
[01:17:55] ------------------------------------------
[01:17:55] 
[01:17:55] thread '[run-fail] run-fail/overflowing-neg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:17:55] 
[01:17:55] ---- [run-fail] run-fail/overflowing-sub.rs stdout ----
[01:17:55]  
[01:17:55] error: compilation failed!
[01:17:55] status: exit code: 101
[01:17:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-sub.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-sub.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-sub.stage2-x86_64-unknown-linux-gnu.aux"
[01:17:55] ------------------------------------------
[01:17:55] 
[01:17:55] ------------------------------------------
[01:17:55] stderr:
[01:17:55] stderr:
[01:17:55] ------------------------------------------
[01:17:55] error: attempt to subtract with overflow
[01:17:55]    |
[01:17:55]    |
[01:17:55] 15 |     let _x = 42u8 - (42u8 + 1);
[01:17:55]    |
[01:17:55]    |
[01:17:55]    = note: #[deny(const_err)] on by default
[01:17:55] error: aborting due to previous error
[01:17:55] 
[01:17:55] 
[01:17:55] ------------------------------------------
---
[01:17:55] test result: FAILED. 133 passed; 4 failed; 1 ignored; 0 measured; 0 filtered out
[01:17:55] 
[01:17:55] 
[01:17:55] 
[01:17:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:55] 
[01:17:55] 
[01:17:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:17:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:55] Build completed unsuccessfully in 0:21:47
[01:17:55] Makefile:58: recipe for target 'check' failed
[01:17:55] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04f73ab4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
