plain
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:09] 
[00:58:09] running 88 tests
[00:59:50] ................................................F........F..................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:02:15] failures:
[01:02:15] 
[01:02:15] ---- [run-pass] run-pass-fulldeps/proc-macro/add-impl.rs stdout ----
[01:02:15]  
[01:02:15]  
[01:02:15] error: compilation failed!
[01:02:15] status: exit code: 101
[01:02:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/add-impl.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/add-impl.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/add-impl.stage2-x86_64-unknown-linux-gnu.aux"
[01:02:15] ------------------------------------------
[01:02:15] 
[01:02:15] ------------------------------------------
[01:02:15] stderr:
[01:02:15] stderr:
[01:02:15] ------------------------------------------
[01:02:15] error[E0658]: procedural macros cannot expand to modules
[01:02:15]   --> /checkout/src/test/run-pass-fulldeps/proc-macro/add-impl.rs:17:10
[01:02:15]    |
[01:02:15] 17 | #[derive(AddImpl)]
[01:02:15]    |
[01:02:15]    = help: add #![feature(proc_macro_mod)] to the crate attributes to enable
[01:02:15] 
[01:02:15] error: aborting due to previous error
---
[01:02:15] ---- [run-pass] run-pass-fulldeps/proc-macro/crate-var.rs stdout ----
[01:02:15]  
[01:02:15] error: compilation failed!
[01:02:15] status: exit code: 101
[01:02:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/crate-var.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/crate-var.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/crate-var.stage2-x86_64-unknown-linux-gnu.aux"
[01:02:15] ------------------------------------------
[01:02:15] 
[01:02:15] ------------------------------------------
[01:02:15] stderr:
[01:02:15] stderr:
[01:02:15] ------------------------------------------
[01:02:15] error[E0658]: procedural macros cannot expand to modules
[01:02:15]   --> /checkout/src/test/run-pass-fulldeps/proc-macro/crate-var.rs:22:14
[01:02:15]    |
[01:02:15] 22 |     #[derive(Double)]
[01:02:15] ...
[01:02:15] ...
[01:02:15] 25 | m!();
[01:02:15]    | ----- in this macro invocation
[01:02:15]    = help: add #![feature(proc_macro_mod)] to the crate attributes to enable
[01:02:15] 
[01:02:15] error: aborting due to previous error
[01:02:15] 
---
[01:02:15] 
[01:02:15] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:02:15] 
[01:02:15] 
[01:02:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:15] 
[01:02:15] 
[01:02:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:15] Build completed unsuccessfully in 0:21:36
[01:02:15] Build completed unsuccessfully in 0:21:36
[01:02:15] Makefile:58: recipe for target 'check' failed
[01:02:15] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ac8030
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
