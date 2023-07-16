\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent.rs","byte_start":1282,"byte_end":1322,"line_start":61,"line_end":61,"column_start":21,"column_end":61,"is_primary":true,"text":[{"text":"    let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);","highlight_start":21,"highlight_end":61}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0161]: cannot move a value of type Dst<A + 'static>: the size of Dst<A + 'static> cannot be statically determined\n  --> /checkout/src/test/ui/trivial-bounds-inconsistent.rs:61:21\n   |\nLL |     let x: Dst<A> = *(Box::new(Dst { x: 1 }) as Box<Dst<A>>);\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:43:22] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:43:22] {"message":"For more information about this error, try `rustc --explain E0161`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0161`.\n"}
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] thread '[ui] ui/trivial-bounds-inconsistent.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:43:22] 
---
[00:43:22] test result: FAILED. 1373 passed; 7 failed; 7 ignored; 0 measured; 0 filtered out
[00:43:22] 
[00:43:22] 
[00:43:22] 
[00:43:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:43:22] 
[00:43:22] 
[00:43:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:22] Build completed unsuccessfully in 0:02:22
[00:43:22] Build completed unsuccessfully in 0:02:22
[00:43:22] Makefile:58: recipe for target 'check' failed
[00:43:22] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ba37d43
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
111308 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
107424 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102820 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102808 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm
102804 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm/s-f0iqj44q13-ei2dfv-v7hez0f2m5v0
90740 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90740 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90736 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0iqgjm9xe-1sl0rmi-35k84lpu7c28x
89716 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89684 ./src/llvm/test/CodeGen
86708 ./obj/build/x86_64-unknown-linux-gnu/doc/core
84368 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
