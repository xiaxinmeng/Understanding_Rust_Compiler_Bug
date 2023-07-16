plain
[00:59:10] ....i............................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:59:20] ...................
[00:59:54] ....................................................................................................
[01:00:25] ......................................................................ii............................
[01:01:17] .................................i...................................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:01:23] .i.ii..........
[01:02:08] ..............................................................................................iiiiii
[01:03:08] ....................................................................................................
[01:03:35] ....................................................................................................
[01:03:53] .................................................................
[01:03:53] test result: ok. 2946 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
---
travis_time:start:test_run-fail
Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:24] 
[01:07:24] running 138 tests
[01:07:35] F...........F..F........................................................F.F.........................
[01:07:39] failures:
[01:07:39] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:07:39] 
[01:07:39] ---- [run-fail] run-fail/adjust_never.rs stdout ----
[01:07:39] ---- [run-fail] run-fail/adjust_never.rs stdout ----
[01:07:39]  
[01:07:39] error: compilation failed!
[01:07:39] status: exit code: 101
[01:07:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/adjust_never.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/adjust_never.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/adjust_never.stage2-x86_64-unknown-linux-gnu.aux"
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] error[E0658]: The `!` type is experimental (see issue #35121)
[01:07:39]    |
[01:07:39]    |
[01:07:39] 15 |     let x: ! = panic!();
[01:07:39]    |
[01:07:39]    |
[01:07:39]    = help: add #![feature(never_type)] to the crate attributes to enable
[01:07:39] error: aborting due to previous error
[01:07:39] 
[01:07:39] For more information about this error, try `rustc --explain E0658`.
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[run-fail] run-fail/adjust_never.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2936:9
[01:07:39] 
[01:07:39] ---- [run-fail] run-fail/call-fn-never-arg.rs stdout ----
[01:07:39]  
[01:07:39] error: compilation failed!
[01:07:39] error: compilation failed!
[01:07:39] status: exit code: 101
[01:07:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/call-fn-never-arg.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/call-fn-never-arg.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/call-fn-never-arg.stage2-x86_64-unknown-linux-gnu.aux"
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] error[E0658]: The `!` type is experimental (see issue #35121)
[01:07:39]    |
[01:07:39]    |
[01:07:39] 17 | fn foo(x: !) -> ! {
[01:07:39]    |
[01:07:39]    |
[01:07:39]    = help: add #![feature(never_type)] to the crate attributes to enable
[01:07:39] error: aborting due to previous error
[01:07:39] 
[01:07:39] For more information about this error, try `rustc --explain E0658`.
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[run-fail] run-fail/call-fn-never-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2936:9
[01:07:39] ---- [run-fail] run-fail/cast-never.rs stdout ----
[01:07:39]  
[01:07:39] error: compilation failed!
[01:07:39] status: exit code: 101
[01:07:39] status: exit code: 101
[01:07:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/cast-never.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/cast-never.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/cast-never.stage2-x86_64-unknown-linux-gnu.aux"
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] error[E0658]: The `!` type is experimental (see issue #35121)
[01:07:39]    |
[01:07:39]    |
[01:07:39] 15 |     let x: ! = panic!();
[01:07:39]    |
[01:07:39]    |
[01:07:39]    = help: add #![feature(never_type)] to the crate attributes to enable
[01:07:39] error: aborting due to previous error
[01:07:39] 
[01:07:39] For more information about this error, try `rustc --explain E0658`.
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[run-fail] run-fail/cast-never.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2936:9
[01:07:39] ---- [run-fail] run-fail/never-associated-type.rs stdout ----
[01:07:39]  
[01:07:39] error: compilation failed!
[01:07:39] status: exit code: 101
[01:07:39] status: exit code: 101
[01:07:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/never-associated-type.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/never-associated-type.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/never-associated-type.stage2-x86_64-unknown-linux-gnu.aux"
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] error[E0658]: The `!` type is experimental (see issue #35121)
[01:07:39]    |
[01:07:39]    |
[01:07:39] 23 |     type Wow = !;
[01:07:39]    |
[01:07:39]    |
[01:07:39]    = help: add #![feature(never_type)] to the crate attributes to enable
[01:07:39] error: aborting due to previous error
[01:07:39] 
[01:07:39] For more information about this error, try `rustc --explain E0658`.
[01:07:39] 
---
[01:07:39] ---- [run-fail] run-fail/never-type-arg.rs stdout ----
[01:07:39]  
[01:07:39] error: compilation failed!
[01:07:39] status: exit code: 101
[01:07:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/never-type-arg.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/never-type-arg.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/never-type-arg.stage2-x86_64-unknown-linux-gnu.aux"
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] error[E0658]: The `!` type is experimental (see issue #35121)
[01:07:39]    |
[01:07:39]    |
[01:07:39] 17 | impl PartialEq<!> for Wub {
[01:07:39]    |
[01:07:39]    |
[01:07:39]    = help: add #![feature(never_type)] to the crate attributes to enable
[01:07:39] 
[01:07:39] error[E0658]: The `!` type is experimental (see issue #35121)
[01:07:39]    |
[01:07:39]    |
[01:07:39] 18 |     fn eq(&self, other: &!) -> bool {
[01:07:39]    |
[01:07:39]    |
[01:07:39]    = help: add #![feature(never_type)] to the crate attributes to enable
[01:07:39] error: aborting due to 2 previous errors
[01:07:39] 
[01:07:39] For more information about this error, try `rustc --explain E0658`.
[01:07:39] 
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[run-fail] run-fail/never-type-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2936:9
[01:07:39] 
[01:07:39] failures:
[01:07:39]     [run-fail] run-fail/adjust_never.rs
[01:07:39]     [run-fail] run-fail/call-fn-never-arg.rs
---
[01:07:39] test result: FAILED. 132 passed; 5 failed; 1 ignored; 0 measured; 0 filtered out
[01:07:39] 
[01:07:39] 
[01:07:39] 
[01:07:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:39] 
[01:07:39] 
[01:07:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:39] Build completed unsuccessfully in 0:19:44
[01:07:39] Build completed unsuccessfully in 0:19:44
[01:07:39] Makefile:58: recipe for target 'check' failed
[01:07:39] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a1bba44
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
