plain
[00:46:19] .........................................................................test [run-pass] run-pass/issue-29227.rs has been running for over 60 seconds
[00:46:27] ...........................
[00:46:49] ....................................................................................................
[00:47:04] ....................................................................................................
[00:47:20] .................F...................................................i..............................
[00:47:57] ....................................................................................................
[00:48:10] ....................................................................................................
[00:48:33] .....................................ii............................................................i
[00:48:53] ....ii....................................................i.ii......................................
[00:48:53] ....ii....................................................i.ii......................................
[00:49:16] ...................................................................iiiiiii..........................
[00:49:30] ...F................................................................................................
[00:49:43] ................F...................................................................................
[00:50:06] ........................................
[00:50:06] failures:
[00:50:06] 
[00:50:06] ---- [run-pass] run-pass/mid-path-type-params.rs stdout ----
[00:50:06] ---- [run-pass] run-pass/mid-path-type-params.rs stdout ----
[00:50:06] 
[00:50:06] error: compilation failed!
[00:50:06] status: exit code: 101
[00:50:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/mid-path-type-params.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mid-path-type-params/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mid-path-type-params/auxiliary"
[00:50:06] ------------------------------------------
[00:50:06] 
[00:50:06] ------------------------------------------
[00:50:06] stderr:
[00:50:06] stderr:
[00:50:06] ------------------------------------------
[00:50:06] error: internal compiler error: librustc_typeck/check/mod.rs:2176: no type for node 71: type f64 (id=71) in fcx 0x7ffdfde5b770
[00:50:06] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[00:50:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:06] error: aborting due to previous error
[00:50:06] 
[00:50:06] 
[00:50:06] 
[00:50:06] note: the compiler unexpectedly panicked. this is a bug.
[00:50:06] 
[00:50:06] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:06] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:50:06] 
[00:50:06] 
[00:50:06] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:50:06] 
[00:50:06] ------------------------------------------
[00:50:06] 
[00:50:06] thread '[run-pass] run-pass/mid-path-type-params.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:50:06] thread '[run-pass] run-pass/mid-path-type-params.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:50:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:06] 
[00:50:06] ---- [run-pass] run-pass/trait-default-method-xc.rs stdout ----
[00:50:06] 
[00:50:06] error: compilation failed!
[00:50:06] status: exit code: 101
[00:50:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/trait-default-method-xc.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/trait-default-method-xc/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/trait-default-method-xc/auxiliary"
[00:50:06] ------------------------------------------
[00:50:06] 
[00:50:06] ------------------------------------------
[00:50:06] stderr:
[00:50:06] stderr:
[00:50:06] ------------------------------------------
[00:50:06] error: internal compiler error: librustc_typeck/check/mod.rs:2176: no type for node 815: type isize (id=815) in fcx 0x7ffe0cb033b0
[00:50:06] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[00:50:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:06] error: aborting due to previous error
[00:50:06] 
[00:50:06] 
[00:50:06] 
[00:50:06] note: the compiler unexpectedly panicked. this is a bug.
[00:50:06] 
[00:50:06] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:06] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:50:06] 
[00:50:06] 
[00:50:06] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:50:06] 
[00:50:06] ------------------------------------------
[00:50:06] 
[00:50:06] thread '[run-pass] run-pass/trait-default-method-xc.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:50:06] thread '[run-pass] run-pass/trait-default-method-xc.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:50:06] 
[00:50:06] ---- [run-pass] run-pass/ufcs-polymorphic-paths.rs stdout ----
[00:50:06] 
[00:50:06] error: compilation failed!
[00:50:06] status: exit code: 101
[00:50:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/ufcs-polymorphic-paths.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/ufcs-polymorphic-paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/ufcs-polymorphic-paths/auxiliary"
[00:50:06] ------------------------------------------
[00:50:06] 
[00:50:06] ------------------------------------------
[00:50:06] stderr:
[00:50:06] stderr:
[00:50:06] ------------------------------------------
[00:50:06] error[E0308]: mismatched types
[00:50:06]    --> /checkout/src/test/run-pass/ufcs-polymorphic-paths.rs:104:5
[00:50:06]     |
[00:50:06] 104 |     Foo::map_in_place::<i8, fn(u8) -> i8>, fn(Foo<u8>, fn(u8) -> i8) -> Foo<i8>,
[00:50:06]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i8, found fn pointer
[00:50:06]     |
[00:50:06]     = note: expected type `fn(Foo<u8>, fn(u8) -> i8) -> Foo<i8>`
[00:50:06]                found type `fn(Foo<_>, _) -> Foo<fn(u8) -> i8> {<Foo<T>><_>::map_in_place::<fn(u8) -> i8, _>}`
[00:50:06] 
[00:50:06] error: internal compiler error: librustc_typeck/check/mod.rs:2176: no type for node 782: type i8 (id=782) in fcx 0x7ffd1236ee00
[00:50:06] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[00:50:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:06] error: aborting due to 2 previous errors
[00:50:06] 
[00:50:06] 
[00:50:06] For more information about this error, try `rustc --explain E0308`.
[00:50:06] 
[00:50:06] note: the compiler unexpectedly panicked. this is a bug.
[00:50:06] 
[00:50:06] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:06] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:50:06] 
[00:50:06] 
[00:50:06] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:50:06] 
[00:50:06] ------------------------------------------
[00:50:06] 
[00:50:06] thread '[run-pass] run-pass/ufcs-polymorphic-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
---
[00:50:06] 
[00:50:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:06] 
[00:50:06] 
[00:50:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:06] 
[00:50:06] 
[00:50:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:06] Build completed unsuccessfully in 0:11:22
[00:50:06] Build completed unsuccessfully in 0:11:22
[00:50:06] make: *** [check] Error 1
[00:50:06] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17e3f433
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
