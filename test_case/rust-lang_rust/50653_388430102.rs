plain
[01:27:55] failures:
[01:27:55] 
[01:27:55] ---- [incremental] incremental/warnings-reemitted.rs stdout ----
[01:27:55]  
[01:27:55] error in revision `cfail1`: test compilation failed although it shouldn't!
[01:27:55] status: exit code: 101
[01:27:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/warnings-reemitted.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Coverflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted.stage2-x86_64-unknown-linux-gnu.aux"
[01:27:55] ------------------------------------------
[01:27:55] 
[01:27:55] ------------------------------------------
[01:27:55] stderr:
[01:27:55] stderr:
[01:27:55] ------------------------------------------
[01:27:55] {"message":"attempt to add with overflow","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/warnings-reemitted.rs","byte_start":595,"byte_end":604,"line_start":18,"line_end":18,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    255u8 + 1; //~ WARNING this expression will panic at run-time","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: attempt to add with overflow\n  --> /checkout/src/test/incremental/warnings-reemitted.rs:18:5\n   |\nLL |     255u8 + 1; //~ WARNING this expression will panic at run-time\n   |     ^^^^^^^^^\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[01:27:55] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:27:55] ------------------------------------------
[01:27:55] 
[01:27:55] thread '[incremental] incremental/warnings-reemitted.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:27:55] 
---
[01:27:55] test result: FAILED. 87 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:27:55] 
[01:27:55] 
[01:27:55] 
[01:27:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:27:55] 
[01:27:55] 
[01:27:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:55] Build completed unsuccessfully in 0:24:30
[01:27:55] Build completed unsuccessfully in 0:24:30
[01:27:55] Makefile:58: recipe for target 'check' failed
[01:27:55] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:023c0fa8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
