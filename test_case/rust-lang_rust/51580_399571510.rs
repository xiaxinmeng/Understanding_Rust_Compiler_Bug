plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:27] 
[01:06:27] running 240 tests
[01:07:45] ....................i.................F.............................................................
[01:08:32] ....F........i......................................................................................
[01:08:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:08:43] ...................................F....
[01:08:43] 
[01:08:43] ---- [rustdoc] rustdoc/ffi.rs stdout ----
[01:08:43] 
[01:08:43] 
[01:08:43] error: htmldocck failed!
[01:08:43] status: exit code: 1
[01:08:43] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi" "/checkout/src/test/rustdoc/ffi.rs"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] 20: @has check failed
[01:08:43]  `XPATH PATTERN` did not match
[01:08:43]      // @has ffi/fn.another.html //pre 'pub unsafe extern "C" fn another(cold_as_ice: u32)'
[01:08:43] Encountered 1 errors
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
[01:08:43] thread '[rustdoc] rustdoc/ffi.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:08:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:43] 
---------
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] 12: @has check failed
[01:08:43]  `XPATH PATTERN` did not match
[01:08:43]      // @has variadic/fn.foo.html //pre 'pub unsafe extern "C" fn foo(x: i32, ...)'
[01:08:43] Encountered 1 errors
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
---
[01:08:43] test result: FAILED. 235 passed; 3 failed; 2 ignored; 0 measured; 0 filtered out
[01:08:43] 
[01:08:43] 
[01:08:43] 
[01:08:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:43] 
[01:08:43] 
[01:08:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:43] Build completed unsuccessfully in 0:24:24
[01:08:43] Build completed unsuccessfully in 0:24:24
[01:08:43] Makefile:58: recipe for target 'check' failed
[01:08:43] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b76d760
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
