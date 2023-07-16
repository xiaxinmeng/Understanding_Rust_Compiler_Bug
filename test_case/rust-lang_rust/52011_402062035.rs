plain
[00:54:39] .........................i..........................................................................
[00:54:43] ................................................................................................i...
[00:54:47] ....................................................................................................
[00:54:54] ....................................................................................................
[00:54:59] ..............................................................................F.....................
[00:55:12] ....................................................................................................
[00:55:20] ....................................................................................................
[00:55:25] .i..................................................................................................
[00:55:32] i..ii...............................................................................................
---
[00:56:13] ---- [compile-fail] compile-fail/issue-32829.rs stdout ----
[00:56:13] 
[00:56:13] error: compiler panicked
[00:56:13] status: exit code: 101
[00:56:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-32829.rs" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32829/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-32829/auxiliary" "-A" "unused"
[00:56:13] ------------------------------------------
[00:56:13] 
[00:56:13] ------------------------------------------
[00:56:13] stderr:
[00:56:13] stderr:
[00:56:13] ------------------------------------------
[00:56:13] error[E0080]: could not evaluate static initializer
[00:56:13]   --> /checkout/src/test/compile-fail/issue-32829.rs:13:22
[00:56:13]    |
[00:56:13] LL | static S : u64 = { { panic!("foo"); 0 } };
[00:56:13]    |                      ^^^^^^^^^^^^^^ the evaluated program panicked at 'foo', /checkout/src/test/compile-fail/issue-32829.rs:13:22
[00:56:13]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:56:13] 
[00:56:13] error[E0080]: could not evaluate static initializer
[00:56:13]   --> /checkout/src/test/compile-fail/issue-32829.rs:13:1
[00:56:13]   --> /checkout/src/test/compile-fail/issue-32829.rs:13:1
[00:56:13]    |
[00:56:13] LL | static S : u64 = { { panic!("foo"); 0 } };
[00:56:13]    | ^^^^^^^^^^^^^^^^^^^^^--------------^^^^^^^
[00:56:13]    |                      |
[00:56:13]    |                      the evaluated program panicked at 'foo', /checkout/src/test/compile-fail/issue-32829.rs:13:22
[00:56:13]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:56:13] 
[00:56:13] error: aborting due to 2 previous errors
[00:56:13] 
---
[00:56:13] 
[00:56:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:13] 
[00:56:13] 
[00:56:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:13] 
[00:56:13] 
[00:56:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:13] Build completed unsuccessfully in 0:14:10
[00:56:13] Build completed unsuccessfully in 0:14:10
[00:56:13] make: *** [check] Error 1
[00:56:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bbf3950
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:19ebd3de:start=1530607742819040321,finish=1530607742825477730,duration=6437409
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c4c11e8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c3fa0be
$ dmesg | grep -i kill
