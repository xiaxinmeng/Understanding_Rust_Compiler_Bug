plain
[00:49:28] ......................................................................i.............................
[00:49:54] ....................................................................................................
[00:50:07] ....................................................................................................
[00:50:30] .......................................ii...........................................................
[00:50:51] .i....ii....................................................i.ii....................................
[00:51:16] .....................................................................iiiiiii........................
[00:51:45] ....................................................................................................
[00:52:00] ....................................................................................................
[00:52:09] .............................................
[00:52:09] test result: ok. 3024 passed; 0 failed; 21 ignored; 0 measured; 0 filtered out
---
[01:26:24] diff of stdout:
[01:26:24] 
[01:26:24] 1 
[01:26:24] 2 running 1 test
[01:26:24] - test src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16) ... FAILED
[01:26:24] + test $DIR/failed-doctest-output.rs - SomeStruct (line 16) ... FAILED
[01:26:24] 5 failures:
[01:26:24] 6 
[01:26:24] 
[01:26:24] 
[01:26:24] - ---- src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16) stdout ----
[01:26:24] - thread 'src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16)' panicked at 'test executable failed:
[01:26:24] + ---- $DIR/failed-doctest-output.rs - SomeStruct (line 16) stdout ----
[01:26:24] + thread '$DIR/failed-doctest-output.rs - SomeStruct (line 16)' panicked at 'test executable failed:
[01:26:24] 9 
[01:26:24] - thread 'main' panicked at 'oh no', src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:26:24] + thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:26:24] 12 
[01:26:24] 13 ', librustdoc/test.rs:367:17
[01:26:24] 
[01:26:24] 15 
[01:26:24] 15 
[01:26:24] 16 
[01:26:24] 17 failures:
[01:26:24] -     src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16)
[01:26:24] +     $DIR/failed-doctest-output.rs - SomeStruct (line 16)
[01:26:24] 20 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:24] 21 
[01:26:24] 
[01:26:24] 
[01:26:24] 
[01:26:24] The actual stdout differed from the expected stdout.
[01:26:24] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:26:24] To update references, rerun the tests and pass the `--bless` flag
[01:26:24] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:26:24] error: 1 errors occurred comparing output.
[01:26:24] status: exit code: 101
[01:26:24] status: exit code: 101
[01:26:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:26:24] ------------------------------------------
[01:26:24] 
[01:26:24] running 1 test
[01:26:24] running 1 test
[01:26:24] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16) ... FAILED
[01:26:24] failures:
[01:26:24] 
[01:26:24] 
[01:26:24] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16) stdout ----
[01:26:24] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16)' panicked at 'test executable failed:
[01:26:24] 
[01:26:24] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:26:24] 
[01:26:24] ', librustdoc/test.rs:367:17
[01:26:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:26:24] 
[01:26:24] 
[01:26:24] 
[01:26:24] failures:
[01:26:24]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 16)
[01:26:24] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:24] 
[01:26:24] 
[01:26:24] ------------------------------------------
---
[01:26:24] test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:24] 
[01:26:24] 
[01:26:24] 
[01:26:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:24] 
[01:26:24] 
[01:26:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:24] Build completed unsuccessfully in 0:45:56
[01:26:24] Build completed unsuccessfully in 0:45:56
[01:26:24] make: *** [check] Error 1
[01:26:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02207653
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
