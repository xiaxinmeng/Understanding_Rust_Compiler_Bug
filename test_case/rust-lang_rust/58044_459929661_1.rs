\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/const-int-overflowing.rs","byte_start":593,"byte_end":608,"line_start":16,"line_end":16,"column_start":30,"column_end":45,"is_primary":true,"text":[{"text":"const NEG_A: (u32, bool) = 0.overflowing_neg();","highlight_start":30,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no method named `overflowing_neg` found for type `{integer}` in the current scope\n  --> /checkout/src/test/run-pass/const-int-overflowing.rs:16:30\n   |\nLL | const NEG_A: (u32, bool) = 0.overflowing_neg();\n   |                              ^^^^^^^^^^^^^^^\n\n"}
[01:05:44] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:05:44] 
[01:05:44] ------------------------------------------
[01:05:44] 
[01:05:44] 
[01:05:44] thread '[run-pass] run-pass/const-int-overflowing.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:44] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:44] 
[01:05:44] ---- [run-pass] run-pass/const-int-wrapping.rs stdout ----
[01:05:44] normalized stderr:
[01:05:44] warning: constant item is never used: `NEG_A`
[01:05:44]    |
[01:05:44]    |
[01:05:44] LL | const NEG_A: u32 = 5u32.wrapping_neg();
[01:05:44]    |
[01:05:44]    = note: #[warn(dead_code)] on by default
[01:05:44] 
[01:05:44] 
[01:05:44] warning: constant item is never used: `NEG_B`
[01:05:44]    |
[01:05:44]    |
[01:05:44] LL | const NEG_B: u32 = 1234567890u32.wrapping_neg();
[01:05:44] 
[01:05:44] 
[01:05:44] 
[01:05:44] 
[01:05:44] 
[01:05:44] The actual stderr differed from the expected stderr.
[01:05:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-int-wrapping/const-int-wrapping.stderr
[01:05:44] To update references, rerun the tests and pass the `--bless` flag
[01:05:44] To only update this specific test, also pass `--test-args const-int-wrapping.rs`
[01:05:44] error: 1 errors occurred comparing output.
[01:05:44] status: exit code: 0
[01:05:44] status: exit code: 0
[01:05:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/const-int-wrapping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-int-wrapping/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/const-int-wrapping/auxiliary"
[01:05:44] ------------------------------------------
[01:05:44] 
[01:05:44] ------------------------------------------
[01:05:44] stderr:
[01:05:44] stderr:
[01:05:44] ------------------------------------------
[01:05:44] {"message":"constant item is never used: `NEG_A`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/const-int-wrapping.rs","byte_start":464,"byte_end":503,"line_start":16,"line_end":16,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"const NEG_A: u32 = 5u32.wrapping_neg();","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: constant item is never used: `NEG_A`\n  --> /checkout/src/test/run-pass/const-int-wrapping.rs:16:1\n   |\nLL | const NEG_A: u32 = 5u32.wrapping_neg();\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:05:44] {"message":"constant item is never used: `NEG_B`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/const-int-wrapping.rs","byte_start":504,"byte_end":552,"line_start":17,"line_end":17,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"const NEG_B: u32 = 1234567890u32.wrapping_neg();","highlight_start":1,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant item is never used: `NEG_B`\n  --> /checkout/src/test/run-pass/const-int-wrapping.rs:17:1\n   |\nLL | const NEG_B: u32 = 1234567890u32.wrapping_neg();\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:44] ------------------------------------------
[01:05:44] 
[01:05:44] thread '[run-pass] run-pass/const-int-wrapping.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:44] 
---
[01:05:44] test result: FAILED. 2937 passed; 2 failed; 7 ignored; 0 measured; 0 filtered out
[01:05:44] 
[01:05:44] 
[01:05:44] 
[01:05:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:44] 
[01:05:44] 
[01:05:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:44] Build completed unsuccessfully in 0:10:25
[01:05:44] Build completed unsuccessfully in 0:10:25
[01:05:44] make: *** [check] Error 1
[01:05:44] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04e0e5fd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 03:04:03 UTC 2019
