plain
[01:04:21] ......................................................................................i.............
[01:04:27] ...................................i................................................................
[01:04:34] ....................................................................................................
[01:04:39] ....................................................................................................
[01:04:43] ..........................................................................F.........................
[01:04:45] failures:
[01:04:45] 
[01:04:45] ---- [compile-fail] compile-fail/weak-lang-item.rs stdout ----
[01:04:45] 
[01:04:45] 
[01:04:45] error: error pattern ' language item required, but not found: `panic_impl`' not found!
[01:04:45] 
[01:04:45] error: error pattern ' language item required, but not found: `eh_personality`' not found!
[01:04:45] 
[01:04:45] error: multiple error patterns not found
[01:04:45] status: exit code: 101
[01:04:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/weak-lang-item.rs" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/weak-lang-item/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/weak-lang-item/auxiliary" "-A" "unused"
[01:04:45] ------------------------------------------
[01:04:45] 
[01:04:45] ------------------------------------------
[01:04:45] stderr:
[01:04:45] stderr:
[01:04:45] ------------------------------------------
[01:04:45] error[E0259]: the name `core` is defined multiple times
[01:04:45]   --> /checkout/src/test/compile-fail/weak-lang-item.rs:18:1
[01:04:45]    |
[01:04:45] LL | extern crate core;
[01:04:45]    | ^^^^^^^^^^^^^^^^^^ `core` reimported here
[01:04:45]    |
[01:04:45]    = note: `core` must be defined only once in the type namespace of this module
[01:04:45] help: You can use `as` to change the binding name of the import
[01:04:45]    |
[01:04:45] LL | extern crate core as other_core;
[01:04:45] 
[01:04:45] 
[01:04:45] error: `#[panic_implementation]` function required, but not found
[01:04:45] 
[01:04:45] error: language item required, but not found: `eh_personality`
[01:04:45] error: aborting due to 3 previous errors
[01:04:45] 
[01:04:45] For more information about this error, try `rustc --explain E0259`.
[01:04:45] 
---
[01:04:45] 
[01:04:45] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:04:45] 
[01:04:45] 
[01:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:45] 
[01:04:45] 
[01:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:45] Build completed unsuccessfully in 0:16:24
[01:04:45] Build completed unsuccessfully in 0:16:24
[01:04:45] Makefile:58: recipe for target 'check' failed
[01:04:45] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06365a9b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
