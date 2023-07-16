\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/missing-allocator.rs","byte_start":699,"byte_end":714,"line_start":23,"line_end":23,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"#[lang = \"oom\"]","highlight_start":1,"highlight_end":16}],"label":"definition of unknown language item `oom`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0522]: definition of an unknown language item: `oom`\n  --> /checkout/src/test/ui/missing-allocator.rs:23:1\n   |\nLL | #[lang = \"oom\"]\n   | ^^^^^^^^^^^^^^^ definition of unknown language item `oom`\n\n"}
[00:46:34] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:34] {"message":"For more information about this error, try `rustc --explain E0522`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0522`.\n"}
[00:46:34] ------------------------------------------
[00:46:34] 
[00:46:34] thread '[ui] ui/missing-allocator.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:46:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:34] 
[00:46:34] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:46:34] 
[00:46:34] 
[00:46:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:34] 
[00:46:34] 
[00:46:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:34] Build completed unsuccessfully in 0:02:07
[00:46:34] Build completed unsuccessfully in 0:02:07
[00:46:34] Makefile:58: recipe for target 'check' failed
[00:46:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d07aff4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
