\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/augmented-assignments.rs","byte_start":384,"byte_end":385,"line_start":21,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y   //~ error: cannot borrow immutable local variable `y` as mutable","highlight_start":5,"highlight_end":6}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to be mutable","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/augmented-assignments.rs","byte_start":302,"byte_end":303,"line_start":18,"line_end":18,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let y = Int(2);","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"mut y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow `y` as mutable, as it is not declared as mutable\n  --> /checkout/src/test/ui/augmented-assignments.rs:21:5\n   |\nLL |     let y = Int(2);\n   |         - help: consider changing this to be mutable: `mut y`\n...\nLL |     y   //~ error: cannot borrow immutable local variable `y` as mutable\n   |     ^ cannot borrow as mutable\n\n"}
[01:24:59] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:24:59] {"message":"Some errors occurred: E0505, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0505, E0596.\n"}
[01:24:59] 
[01:24:59] ------------------------------------------
[01:24:59] 
[01:24:59] thread '[ui (nll)] ui/augmented-assignments.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:24:59] 
[01:24:59] 
[01:24:59] 
[01:24:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:24:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:24:59] 
[01:24:59] 
[01:24:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:59] Build completed unsuccessfully in 0:08:01
[01:24:59] Build completed unsuccessfully in 0:08:01
[01:24:59] Makefile:48: recipe for target 'check' failed
[01:24:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11f6ff66
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 05:48:32 UTC 2019
---
travis_time:end:020d60c0:start=1549604914970156663,finish=1549604914979589217,duration=9432554
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01dda0d5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16fb1605
travis_time:start:16fb1605
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:20152dac
$ dmesg | grep -i kill
