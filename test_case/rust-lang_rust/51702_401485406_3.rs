\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/infinite_loop.rs","byte_start":970,"byte_end":980,"line_start":21,"line_end":21,"column_start":20,"column_end":30,"is_primary":false,"text":[{"text":"            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };","highlight_start":20,"highlight_end":30}],"label":"duplicate interpreter state observed here, const evaluation will never terminate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/const-eval/infinite_loop.rs","byte_start":666,"byte_end":1032,"line_start":16,"line_end":24,"column_start":18,"column_end":6,"is_primary":true,"text":[{"text":"    let _ = [(); {","highlight_start":18,"highlight_end":19},{"text":"        //~^ WARNING Constant evaluating a complex constant, this might take some time","highlight_start":1,"highlight_end":87},{"text":"        //~| ERROR could not evaluoccurred: E0019, E0080.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0019, E0080.\n"}
[00:48:21] {"message":"For more information about an error, try `rustc --explain E0019`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0019`.\n"}
[00:48:21] ------------------------------------------
[00:48:21] 
[00:48:21] thread '[ui] ui/const-eval/infinite_loop.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:48:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:21] test result: FAILED. 1529 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[00:48:21] 
[00:48:21] 
[00:48:21] 
[00:48:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:21] 
[00:48:21] 
[00:48:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:21] Build completed unsuccessfully in 0:02:14
[00:48:21] Build completed unsuccessfully in 0:02:14
[00:48:21] Makefile:58: recipe for target 'check' failed
[00:48:21] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01741b04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
