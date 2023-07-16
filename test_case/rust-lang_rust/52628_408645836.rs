plain
[01:20:02] 
[01:20:02] 12 3 | no
[01:20:02] 13   | ^^ not found in this scope
[01:20:02] 14 
[01:20:02] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 26)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:20:02] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 26)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:20:02] 17 
[01:20:02] 17 
[01:20:02] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 20) stdout ----
[01:20:02] 
[01:20:02] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:20:02] 23 
[01:20:02] - ', librustdoc/test.rs:367:17
[01:20:02] + ', librustdoc/test.rs:368:17
[01:20:02] 25 
[01:20:02] 25 
[01:20:02] 26 
[01:20:02] 27 failures:
[01:20:02] 
[01:20:02] 
[01:20:02] The actual stdout differed from the expected stdout.
[01:20:02] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:20:02] To update references, rerun the tests and pass the `--bless` flag
[01:20:02] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:20:02] error: 1 errors occurred comparing output.
[01:20:02] status: exit code: 101
[01:20:02] status: exit code: 101
[01:20:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:20:02] ------------------------------------------
[01:20:02] 
[01:20:02] running 2 tests
[01:20:02] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:20:02] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) ... FAILED
[01:20:02] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) ... FAILED
[01:20:02] failures:
[01:20:02] 
[01:20:02] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:20:02] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26) stdout ----
[01:20:02] error[E0425]: cannot find value `no` in this scope
[01:20:02]   |
[01:20:02] 3 | no
[01:20:02]   | ^^ not found in this scope
[01:20:02] 
[01:20:02] 
[01:20:02] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 26)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:20:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:20:02] 
[01:20:02] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20) stdout ----
[01:20:02] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 20)' panicked at 'test executab/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:02] 
[01:20:02] 
[01:20:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:02] Build completed unsuccessfully in 0:34:30
[01:20:02] Build completed unsuccessfully in 0:34:30
[01:20:02] Makefile:58: recipe for target 'check' failed
[01:20:02] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09df34f2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
