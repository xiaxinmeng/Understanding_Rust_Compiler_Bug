plain
[00:49:51] ....................................................................................................
[00:49:53] ....................................................................................................
[00:49:56] ....................................................................................................
[00:50:00] ........i...........................................................................................
[00:50:05] ...............................................................................FFFF.F...............
[00:50:11] .................ii.iii.............................................................................
[00:50:14] ....................................................................................................
[00:50:16] ....................................................................................................
[00:50:19] ....................................................................................................
---
[00:51:50] normalized stderr:
[00:51:50] error[E0308]: mismatched types
[00:51:50]   --> $DIR/const-int-overflowing.rs:12:27
[00:51:50]    |
[00:51:50] LL |     let x: &'static i32 = &(5_i32.overflowing_add(3)); //~ ERROR does not live long enough
[00:51:50]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found tuple
[00:51:50]    = note: expected type `&'static i32`
[00:51:50]    = note: expected type `&'static i32`
[00:51:50]               found type `&(i32, bool)`
[00:51:50] error[E0308]: mismatched types
[00:51:50]   --> $DIR/const-int-overflowing.rs:13:27
[00:51:50]    |
[00:51:50]    |
[00:51:50] LL |     let y: &'static i32 = &(5_i32.overflowing_sub(3)); //~ ERROR does not live long enough
[00:51:50]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found tuple
[00:51:50]    = note: expected type `&'static i32`
[00:51:50]    = note: expected type `&'static i32`
[00:51:50]               found type `&(i32, bool)`
[00:51:50] error[E0308]: mismatched types
[00:51:50]   --> $DIR/const-int-overflowing.rs:14:27
[00:51:50]    |
[00:51:50]    |
[00:51:50] LL |     let z: &'static i32 = &(5_i32.overflowing_mul(3)); //~ ERROR does not live long enough
[00:51:50]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found tuple
[00:51:50]    = note: expected type `&'static i32`
[00:51:50]    = note: expected type `&'static i32`
[00:51:50]               found type `&(i32, bool)`
[00:51:50] error: aborting due to 3 previous errors
[00:51:50] 
[00:51:50] For more information about this error, try `rustc --explain E0308`.
[00:51:50] 
[00:51:50] 
[00:51:50] 
[00:51:50] 
[00:51:50] The actual stderr differed from the expected stderr.
[00:51:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/const-int-overflowing.stderr
[00:51:50] To update references, rerun the tests and pass the `--bless` flag
[00:51:50] To only update this specific test, also pass `--test-args consts/const-int-overflowing.rs`
[00:51:50] error: 1 errors occurred comparing output.
[00:51:50] status: exit code: 1
[00:51:50] status: exit code: 1
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-overflowing.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/auxiliary" "-A" "unused"
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] 
[0: &'static i32 = &(5_i32.overflowing_mul(3)); //~ ERROR does not live long enough\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found tuple\n   |\n   = note: expected type `&'static i32`\n              found type `&(i32, bool)`\n\n"}
[00:51:50] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:51:50] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] thread '[ui] ui/consts/const-int-overflowing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:50] 
[00:51:50] 
[00:51:50] ---- [ui] ui/consts/const-int-rotate.rs stdout ----
[00:51:50] 
[00:51:50] error: ui test compiled successfully!
[00:51:50] status: exit code: 0
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-rotate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/auxiliary" "-A" "unused"
[00:51:50] ----------------------------------exit code: 0
[00:51:50] ----------------------------------exit code: 0
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-wrapping.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping/auxiliary" "-A" "unused"
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] ------------------------------------------
[00:51:50] stderr:
---
[00:51:50] 
[00:51:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:51:50] 
[00:51:50] 
[00:51:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:50] 
[00:51:50] 
[00:51:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:50] Build completed unsuccessfully in 0:03:16
[00:51:50] Build completed unsuccessfully in 0:03:16
[00:51:50] Makefile:58: recipe for target 'check' failed
[00:51:50] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:29fca202
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
74068 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
74064 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
72140 ./src/llvm/lib
71888 ./obj/build/x86_64-unknown-linux-gnu/doc/std
71300 ./obj.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02fb4fd9
travis_time:start:02fb4fd9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08170844
$ dmesg | grep -i kill
