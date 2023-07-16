plain
[01:18:43] failures:
[01:18:43] 
[01:18:43] ---- [compile-fail] compile-fail/linkage3.rs stdout ----
[01:18:43]  
[01:18:43] error: Error: expected failure status (Some(101)) but received status None.
[01:18:43] status: signal: 6
[01:18:43] command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/linkage3.rs" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/compile-fail" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/i686-unknown-linux-gnu/test/compile-fail/linkage3.stage2-i686-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/compile-fail/linkage3.stage2-i686-unknown-linux-gnu.aux" "-A" "unused"
[01:18:43] ------------------------------------------
[01:18:43] 
[01:18:43] ------------------------------------------
[01:18:43] stderr:
[01:18:43] stderr:
[01:18:43] ------------------------------------------
[01:18:43] {"message":"invalid linkage specified","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/compile-fail/linkage3.rs","byte_start":521,"byte_end":544,"line_start":14,"line_end":14,"column_start":24,"column_end":47,"is_primary":true,"text":[{"text":"    #[linkage = \"foo\"] static foo: *const i32;","highlight_start":24,"highlight_end":47}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: invalid linkage specified\n  --> /checkout/src/test/compile-fail/linkage3.rs:14:24\n   |\nLL |     #[linkage = \"foo\"] static foo: *const i32;\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:18:43] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:18:43] rustc: cxa_atexit.c:100: __new_exitfn: Assertion `l != NULL' failed.
[01:18:43] ------------------------------------------
[01:18:43] 
[01:18:43] thread '[compile-fail] compile-fail/linkage3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:18:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:18:43] 
[01:18:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:18:43] 
[01:18:43] 
[01:18:43] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "compile-fail" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:43] 
[01:18:43] 
[01:18:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:43] Build completed unsuccessfully in 1:13:58
