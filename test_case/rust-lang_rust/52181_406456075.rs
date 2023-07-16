plain
[01:20:01] 
[01:20:01] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:20:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:20:01] 
[01:20:01] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:20:01] status: exit code: 101
[01:20:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:20:01] ------------------------------------------
[01:20:01] 
[01:20:01] running 2 tests
[01:20:01] running 2 tests
[01:20:01] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 25) ... FAILED
[01:20:01] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 19) ... FAILED
[01:20:01] failures:
[01:20:01] 
[01:20:01] 
[01:20:01] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 25) stdout ----
[01:20:01] error[E0425]: cannot find value `no` in this scope
[01:20:01]  --> /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:26:1
[01:20:01] 3 | no
[01:20:01]   | ^^ not found in this scope
[01:20:01] 
[01:20:01] 
[01:20:01] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 25)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:20:01] 
[01:20:01] 
[01:20:01] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 19) stdout ----
[01:20:01] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 19)' panicked at 'test executable failed:
[01:20:01] 
[01:20:01] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:20:01] 
[01:20:01] ', librustdoc/test.rs:367:17
[01:20:01] 
[01:20:01] 
[01:20:01] 
[01:20:01] failures:
[01:20:01]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 25)
[01:20:01]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 19)
[01:20:01] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:20:01] 
[01:20:01] 
[01:20:01] ------------------------------------------
---
[01:20:01] test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:20:01] 
[01:20:01] 
[01:20:01] 
[01:20:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:01] 
[01:20:01] 
[01:20:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:01] Build completed unsuccessfully in 0:36:27
[01:20:01] Build completed unsuccessfully in 0:36:27
[01:20:01] make: *** [check] Error 1
[01:20:01] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:162ad3e2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:17572ef9:start=1532047655842468577,finish=1532047655850951937,duration=8483360
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:058b370b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00bf97ba
travis_time:start:00bf97ba
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b89bc36
$ dmesg | grep -i kill
