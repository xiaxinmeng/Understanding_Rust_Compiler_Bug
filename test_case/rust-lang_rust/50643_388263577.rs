plain
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:21] 
[01:00:21] running 2311 tests
[01:00:26] ....................................................................................................
[01:00:32] ................................................................F...................................
[01:00:38] .......................................F..............F.............................................
[01:00:51] ....................i............................................................ii.iii.............
[01:00:58] ....................................................................................................
[01:01:05] ..........i..............................i..........................................................
[01:01:11] ....................................................................................................
---
[01:03:20] failures:
[01:03:20] 
[01:03:20] ---- [compile-fail] compile-fail/borrowck/borrowck-field-sensitivity.rs stdout ----
[01:03:20]  
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-field-sensitivity.rs:24: expected error not found: use of moved value: `*x.b`
[01:03:20] 
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-field-sensitivity.rs:37: expected error not found: use of moved value: `x.b`
[01:03:20] 
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-field-sensitivity.rs:51: expected error not found: cannot move out of `x.b` because it is borrowed
[01:03:20] 
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-field-sensitivity.rs:72: expected error not found: use of moved value: `x.b`
[01:03:20] 
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-field-sensitivity.rs:78: expected error not found: use of moved value: `x.b`
[01:03:20] 
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-field-sensitivity.rs:84: expected error not found: use of moved value: `x.b`
[01:03:20] 
[01:03:20] error: 0 unexpected errors found, 6 expected errors not found
[01:03:20] status: exit code: 101
[01:03:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/borrowck/borrowck-field-sensitivity.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-forma "use of moved value: `x.b`"
[01:03:20]     Error {
[01:03:20]         line_num: 84,
[01:03:20]         kind: Some(
[01:03:20]             Error
[01:03:20]             Error
[01:03:20]         ),
[01:03:20]         msg: "use of moved value: `x.b`"
[01:03:20] ]
[01:03:20] 
[01:03:20] thread '[compile-fail] compile-fail/borrowck/borrowck-field-sensitivity.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:03:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:20] 
[01:03:20] ---- [compile-fail] compile-fail/borrowck/borrowck-struct-update-with-dtor.rs stdout ----
[01:03:20]  
[01:03:20] error in revision `ast`: /checkout/src/test/compile-fail/borrowck/borrowck-struct-update-with-dtor.rs:31: expected error not found: cannot move out of type `T`, which implements the `Drop` trait
[01:03:20] 
[01:03:20] error in revision `ast`: 0 unexpected errors found, 1 expected errors not found
[01:03:20] status: exit code: 101
[01:03:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/borrowck/borrowck-struct-update-with-dtor.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/borrowck-struct-update-with-dtor.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/borrowck-struct-update-with-dtor.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:03:20] not found errors (from test file): [
[01:03:20]     Error {
[01:03:20]         line_num: 31,
[01:03:20]         kind: Some(
[01:03:20]         ),
[01:03:20]         ),
[01:03:20]         msg: "cannot move out of type `T`, which implements the `Drop` trait"
[01:03:20] ]
[01:03:20] 
[01:03:20] thread '[compile-fail] compile-fail/borrowck/borrowck-struct-update-with-dtor.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:03:20] 
[01:03:20] 
[01:03:20] ---- [compile-fail] compile-fail/borrowck/borrowck-use-mut-borrow.rs stdout ----
[01:03:20]  
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-use-mut-borrow.rs:49: expected error not found: cannot use `x.a` because it was mutably borrowed
[01:03:20] 
[01:03:20] error: /checkout/src/test/compile-fail/borrowck/borrowck-use-mut-borrow.rs:57: expected error not found: cannot use `x.a` because it was mutably borrowed
[01:03:20] 
[01:03:20] error: 0 unexpected errors found, 2 expected errors not found
[01:03:20] status: exit code: 101
[01:03:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/borrowck/borrowck-use-mut-borrow.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/borrowck-use-mut-borrow.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/borrowck/borrowck-use-mut-borrow.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:03:20] not found errors (from test file): [
[01:03:20]     Error {
[01:03:20]         line_num: 49,
[01:03:20]         kind: Some(
[01:03:20]         ),
[01:03:20]         ),
[01:03:20]         msg: "cannot use `x.a` because it was mutably borrowed"
[01:03:20]     Error {
[01:03:20]         line_num: 57,
[01:03:20]         kind: Some(
[01:03:20]             Error
[01:03:20]             Error
[01:03:20]         ),
[01:03:20]         msg: "cannot use `x.a` because it was mutably borrowed"
[01:03:20] ]
[01:03:20] 
[01:03:20] thread '[compile-fail] compile-fail/borrowck/borrowck-use-mut-borrow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1312:13
[01:03:20] 
---
[01:03:20] test result: FAILED. 2293 passed; 3 failed; 15 ignored; 0 measured; 0 filtered out
[01:03:20] 
[01:03:20] 
[01:03:20] 
[01:03:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:20] 
[01:03:20] 
[01:03:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:20] Build completed unsuccessfully in 0:17:05
[01:03:20] Build completed unsuccessfully in 0:17:05
[01:03:20] Makefile:58: recipe for target 'check' failed
[01:03:20] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:008c38f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
