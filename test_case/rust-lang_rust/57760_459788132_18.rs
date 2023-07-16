\n\nCertain Rust types must be cast before passing them to a variadic function,\nbecause of arcane ABI rules dictated by the C standard. To fix the error,\ncast the value to the type specified by the error message (which you may need\nto import from `std::os::raw`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/variadic/variadic-ffi-3.rs","byte_start":1172,"byte_end":1176,"line_start":29,"line_end":29,"column_start":19,"column_end":23,"is_primary":true,"text":[{"text":"        foo(1, 2, 1u16); //~ ERROR can't pass `u16` to variadic function","highlight_start":19,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"cast the value to `c_uint`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/variadic/variadic-ffi-3.rs","byte_start":1172,"byte_end":1176,"line_start":29,"line_end":29,"column_start":19,"column_end":23,"is_primary":true,"text":[{"text":"        foo(1, 2, 1u16); //~ ERROR can't pass `u16` to variadic function","highlight_start":19,"highlight_end":23}],"label":null,"suggested_replacement":"1u16 as c_uint","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0617]: can't pass `u16` to variadic function\n  --> /checkout/src/test/ui/variadic/variadic-ffi-3.rs:29:19\n   |\nLL |         foo(1, 2, 1u16); //~ ERROR can't pass `u16` to variadic function\n   |                   ^^^^ help: cast the value to `c_uint`: `1u16 as c_uint`\n\n"}
[01:02:39] {"message":"aborting due to 10 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 10 previous errors\n\n"}
[01:02:39] {"message":"Some errors occurred: E0060, E0308, E0617.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0060, E0308, E0617.\n"}
[01:02:39] 
[01:02:39] ------------------------------------------
[01:02:39] 
[01:02:39] thread '[ui] ui/variadic/variadic-ffi-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:02:39] 
[01:02:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:02:39] 
[01:02:39] 
[01:02:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:39] 
[01:02:39] 
[01:02:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:39] Build completed unsuccessfully in 0:04:05
[01:02:39] Build completed unsuccessfully in 0:04:05
[01:02:39] make: *** [check] Error 1
[01:02:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06838e6d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 16:48:59 UTC 2019
---
travis_time:end:1325f658:start=1549039740978872858,finish=1549039740983833554,duration=4960696
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18884dd0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!check
