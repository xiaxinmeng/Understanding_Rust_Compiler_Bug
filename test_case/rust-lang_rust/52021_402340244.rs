plain
[00:50:49] ....................................................................................................
[00:50:55] ....................................................................................................
[00:51:01] ....................................................................................................
[00:51:09] ....................................................................................................
[00:51:14] i........................................................................................FFFF......i
[00:51:21] ..ii................................................................................................
[00:51:34] ....................................................................................................
[00:51:34] ....................................................................................................
[00:51:39] ...................................F...............................................i................
[00:51:50] ....................................................................................................
[00:51:55] ....................................................................................................
errors (from JSON output): [
[00:52:00]     Error {
---
[00:52:00]         line_num: 305,
[00:52:00]         kind: Some(
[00:52:00]             Error
[00:52:00]         ),
[00:52:00]         msg: "free region `` does not outlive"
[00:52:00] ]
[00:52:00] 
[00:52:00] thread '[compile-fail] compile-fail/borrowck/borrowck-describe-lvalue.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:52:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:00] 
[00:52:00] ---- [compile-fail] compile-fail/mir_check_cast_closure.rs stdout ----
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_closure.rs:16: unexpected error: '16:28: 16:37: unsatisfied lifetime constraints'
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_closure.rs:16: expected error not found: free region `'b` does not outlive free region `'a`
[00:52:00] error: 1 unexpected errors found, 1 expected errors not found
[00:52:00] status: exit code: 101
[00:52:00] status: exit code: 101
[00:52:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/mir_check_cast_closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_closure/auxiliary" "-A" "unused"
[00:52:00]     Error {
[00:52:00]         line_num: 16,
[00:52:00]         kind: Some(
[00:52:00]             Error
---
[00:52:00]         line_num: 16,
[00:52:00]         kind: Some(
[00:52:00]             Error
[00:52:00]         ),
[00:52:00]         msg: "free region `\'b` does not outlive free region `\'a`"
[00:52:00] ]
[00:52:00] 
[00:52:00] thread '[compile-fail] compile-fail/mir_check_cast_closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:52:00] 
[00:52:00] 
[00:52:00] ---- [compile-fail] compile-fail/mir_check_cast_reify.rs stdout ----
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_reify.rs:46: unexpected error: '46:25: 46:28: unsatisfied lifetime constraints'
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_reify.rs:46: expected error not found: free region `'a` does not outlive free region `'static`
[00:52:00] error: 1 unexpected errors found, 1 expected errors not found
[00:52:00] status: exit code: 101
[00:52:00] status: exit code: 101
[00:52:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/mir_check_cast_reify.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_reify/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_reify/auxiliary" "-A" "unused"
[00:52:00]     Error {
[00:52:00]         line_num: 46,
[00:52:00]         kind: Some(
[00:52:00]             Error
---
[00:52:00]         line_num: 46,
[00:52:00]         kind: Some(
[00:52:00]             Error
[00:52:00]         ),
[00:52:00]         msg: "free region `\'a` does not outlive free region `\'static`"
[00:52:00] ]
[00:52:00] 
[00:52:00] thread '[compile-fail] compile-fail/mir_check_cast_reify.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:52:00] 
[00:52:00] 
[00:52:00] ---- [compile-fail] compile-fail/mir_check_cast_unsafe_fn.rs stdout ----
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_unsafe_fn.rs:18: unexpected error: '18:32: 18:33: unsatisfied lifetime constraints'
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_unsafe_fn.rs:18: expected error not found: free region `'a` does not outlive free region `'static`
[00:52:00] error: 1 unexpected errors found, 1 expected errors not found
[00:52:00] status: exit code: 101
[00:52:00] status: exit code: 101
[00:52:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/mir_check_cast_unsafe_fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_unsafe_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_unsafe_fn/auxiliary" "-A" "unused"
[00:52:00]     Error {
[00:52:00]         line_num: 18,
[00:52:00]         kind: Some(
[00:52:00]             Error
---
[00:52:00]         line_num: 18,
[00:52:00]         kind: Some(
[00:52:00]             Error
[00:52:00]         ),
[00:52:00]         msg: "free region `\'a` does not outlive free region `\'static`"
[00:52:00] ]
[00:52:00] 
[00:52:00] thread '[compile-fail] compile-fail/mir_check_cast_unsafe_fn.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:52:00] 
[00:52:00] 
[00:52:00] ---- [compile-fail] compile-fail/mir_check_cast_unsize.rs stdout ----
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_unsize.rs:19: unexpected error: '19:5: 19:6: unsatisfied lifetime constraints'
[00:52:00] 
[00:52:00] error: /checkout/src/test/compile-fail/mir_check_cast_unsize.rs:17: expected error not found: free region `'a` does not outlive free region `'static`
[00:52:00] error: 1 unexpected errors found, 1 expected errors not found
[00:52:00] status: exit code: 101
[00:52:00] status: exit code: 101
[00:52:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/mir_check_cast_unsize.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_unsize/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/mir_check_cast_unsize/auxiliary" "-A" "unused"
[00:52:00]     Error {
[00:52:00]         line_num: 19,
[00:52:00]         kind: Some(
[00:52:00]             Error
---
[00:52:00]         line_num: 17,
[00:52:00]         kind: Some(
[00:52:00]             Error
[00:52:00]         ),
[00:52:00]         msg: "free region `\'a` does not outlive free region `\'static`"
[00:52:00] ]
[00:52:00] 
[00:52:00] thread '[compile-fail] compile-fail/mir_check_cast_unsize.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:52:00] 
[00:52:00] 
[00:52:00] ---- [compile-fail] compile-fail/regions-static-bound.rs#nll stdout ----
[00:52:00] 
[00:52:00] error in revision `nll`: /checkout/src/test/compile-fail/regions-static-bound.rs:19: unexpected error: '19:5: 19:6: unsatisfied lifetime constraints'
[00:52:00] 
[00:52:00] error in revision `nll`: /checkout/src/test/compile-fail/regions-static-bound.rs:19: expected error not found: free region `'a` does not outlive free region `'static`
[00:52:00] 
[00:52:00] error in revision `nll`: 1 unexpected errors found, 1 expected errors not found
[00:52:00] status: exit code: 101
[00:52:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/regions-static-bound.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/regions-static-bound.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/regions-static-bound.nll/auxiliary" "-A" "unused"
[00:52:00]     Error {
[00:52:00]         line_num: 19,
[00:52:00]         kind: Some(
[00:52:00]             Error
---
[00:52:00]         line_num: 19,
[00:52:00]         kind: Some(
[00:52:00]             Error
[00:52:00]         ),
[00:52:00]         msg: "free region `\'a` does not outlive free region `\'static`"
[00:52:00] ]
[00:52:00] 
[00:52:00] thread '[compile-fail] compile-fail/regions-static-bound.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:52:00] 
---
[00:52:00] 
[00:52:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:52:00] 
[00:52:00] 
[00:52:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:00] 
[00:52:00] 
[00:52:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:00] Build completed unsuccessfully in 0:13:33
[00:52:00] Build completed unsuccessfully in 0:13:33
[00:52:00] Makefile:58: recipe for target 'check' failed
[00:52:00] make: *** [check] Error 1
57324 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
52728 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
52028 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
51848 ./obj/build/x86_64-unknown-linux-gnu/test/ui
