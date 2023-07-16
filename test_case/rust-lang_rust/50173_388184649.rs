plain
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:47] 
[00:51:47] running 2980 tests
[00:52:04] ................F...................................................................................
[00:52:41] ....................................................................................................
[00:52:57] ....................................................................................................
[00:53:13] ....................................................................................................
[00:53:37] ....................................................................................................
---
[00:58:30] .........test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:59:00] ...........................................................................................
[00:59:20] ..................................................................................ii................
[01:00:18] ..............................................i....................................................i
[01:00:22] .ii.......test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:01:21] .......iiiiiii......................................................................................
[01:01:43] ....................................................................................................
[01:02:03] ....................................................................................................
[01:02:23] ................................................................................
[01:02:23] ................................................................................
[01:02:23] failures:
[01:02:23] 
[01:02:23] ---- [run-pass] run-pass/arbitrary_self_types_pointers_and_wrappers.rs stdout ----
[01:02:23] error: compilation failed!
[01:02:23] status: exit code: 101
[01:02:23] status: exit code: 101
[01:02:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/arbitrary_self_types_pointers_and_wrappers.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_pointers_and_wrappers.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_pointers_and_wrappers.stage2-x86_64-unknown-linux-gnu.aux"
[01:02:23] ------------------------------------------
[01:02:23] 
[01:02:23] ------------------------------------------
[01:02:23] stderr:
[01:02:23] stderr:
[01:02:23] ------------------------------------------
[01:02:23] error[E0599]: no method named `ptr_wrapper` found for type `Ptr<Wrapper<std::fmt::Debug>>` in the current scope
[01:02:23]   --> /checkout/src/test/run-pass/arbitrary_self_types_pointers_and_wrappers.rs:64:19
[01:02:23]    |
[01:02:23] 18 | struct Ptr<T: ?Sized>(Box<T>);
[01:02:23]    | ------------------------------ method `ptr_wrapper` not found for this
[01:02:23] ...
[01:02:23] 64 |     assert_eq!(pw.ptr_wrapper(), 5);
[01:02:23]    |
[01:02:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:02:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:02:23]    = note: the following trait defines an item `ptr_wrapper`, perhaps you need to implement it:
[01:02:23]            candidate #1: `Trait`
[01:02:23] 
[01:02:23] error[E0599]: no method named `wrapper_ptr_wrapper` found for type `Wrapper<Ptr<Wrapper<std::fmt::Debug>>>` in the current scope
[01:02:23]   --> /checkout/src/test/run-pass/arbitrary_self_types_pointers_and_wrappers.rs:67:20
[01:02:23]    |
[01:02:23] 30 | struct Wrapper<T: ?Sized>(T);
[01:02:23]    | ----------------------------- method `wrapper_ptr_wrapper` not found for this
[01:02:23] ...
[01:02:23] 67 |     assert_eq!(wpw.wrapper_ptr_wrapper(), 6);
[01:02:23]    |
[01:02:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:02:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:02:23]    = note: the following trait defines an item `wrapper_ptr_wrapper`, perhaps you need to implement it:
[01:02:23]            candidate #1: `Trait`
[01:02:23] error: aborting due to 2 previous errors
[01:02:23] 
[01:02:23] For more information about this error, try `rustc --explain E0599`.
[01:02:23] 
[01:02:23] 
[01:02:23] ------------------------------------------
[01:02:23] 
[01:02:23] thread '[run-pass] run-pass/arbitrary_self_types_pointers_and_wrappers.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:02:23] 
[01:02:23] 
[01:02:23] failures:
[01:02:23]     [run-pass] run-pass/arbitrary_self_types_pointers_and_wrappers.rs
[01:02:23]     [run-pass] run-pass/arbitrary_self_types_pointers_and_wrappers.rs
[01:02:23] 
[01:02:23] test result: FAILED. 2960 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out
[01:02:23] 
[01:02:23] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:02:23] 
[01:02:23] 
[01:02:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:23] 
[01:02:23] 
[01:02:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:23] Build completed unsuccessfully in 0:14:42
[01:02:23] Build completed unsuccessfully in 0:14:42
[01:02:23] make: *** [check] Error 1
[01:02:23] Makefile:58: recipe for target 'check' failed
112620 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
112616 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
108624 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
103728 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp
103728 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp
103724 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp/s-f0wzxahbd3-1pt9yg9-1nozvcbv9keul
98508 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
93500 ./obj/build/x86_64-unknown-linux-gnu/stage1
93476 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90844 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90844 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90840 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0wzurp8ig-1ylfq8i-2w93x474pe4e6
87816 ./obj/build/x86_64-unknown-linux-gnu/doc/core
81436 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
80984 ./obj/build/x86_64-unknown-linux-gnu/doc/std
79036 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
---
60840 ./src/llvm-emscripten/lib
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55448 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53676 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33ta18b3panbi
53672 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33ta18b3panbi/s-f0wzwhtunt-etfjlr-2tybn1l6ckot6
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47984 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
47104 ./src/test
46812 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
