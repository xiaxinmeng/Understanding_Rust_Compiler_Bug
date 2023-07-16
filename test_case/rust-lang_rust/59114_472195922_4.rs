\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/dropck_tarena_cycle_checked.rs","byte_start":2531,"byte_end":2537,"line_start":116,"line_end":116,"column_start":7,"column_end":13,"is_primary":true,"text":[{"text":"    f(&arena);","highlight_start":7,"highlight_end":13}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui-fulldeps/dropck_tarena_cycle_checked.rs","byte_start":2540,"byte_end":2541,"line_start":117,"line_end":117,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"} //~^ ERROR `arena` does not live long enough","highlight_start":1,"highlight_end":2}],"label":"`arena` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui-fulldeps/dropck_tarena_cycle_checked.rs","byte_start":2540,"byte_end":2541,"line_start":117,"line_end":117,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"} //~^ ERROR `arena` does not live long enough","highlight_start":1,"highlight_end":2}],"label":"borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `arena::TypedArena`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `arena` does not live long enough\n  --> /checkout/src/test/ui-fulldeps/dropck_tarena_cycle_checked.rs:116:7\n   |\nLL |     f(&arena);\n   |       ^^^^^^ borrowed value does not live long enough\nLL | } //~^ ERROR `arena` does not live long enough\n   | -\n   | |\n   | `arena` dropped here while still borrowed\n   | borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `arena::TypedArena`\n\n"}
[01:21:43] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:21:43] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[01:21:43] 
[01:21:43] ------------------------------------------
---
[01:21:43] test result: FAILED. 18 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:21:43] 
[01:21:43] 
[01:21:43] 
[01:21:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:43] 
[01:21:43] 
[01:21:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:43] Build completed unsuccessfully in 0:12:25
[01:21:43] Build completed unsuccessfully in 0:12:25
[01:21:43] make: *** [check] Error 1
[01:21:43] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:025555c2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 21:52:35 UTC 2019
---
travis_time:end:05e85a84:start=1552427556937398663,finish=1552427556941921642,duration=4522979
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:194b913a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2af241f4
travis_time:start:2af241f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a50a234
$ dmesg | grep -i kill
