\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/futures-api.rs","byte_start":216,"byte_end":220,"line_start":12,"line_end":12,"column_start":11,"column_end":15,"is_primary":true,"text":[{"text":"    Poll, Wake, Waker, LocalWaker,","highlight_start":11,"highlight_end":15}],"label":"no `Wake` in `task`. Did you mean to use `Waker`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/futures-api.rs","byte_start":229,"byte_end":239,"line_start":12,"line_end":12,"column_start":24,"column_end":34,"is_primary":true,"text":[{"text":"    Poll, Wake, Waker, LocalWaker,","highlight_start":24,"highlight_end":34}],"label":"no `LocalWaker` in `task`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/futures-api.rs","byte_start":245,"byte_end":256,"line_start":13,"line_end":13,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    local_waker, local_waker_from_nonlocal,","highlight_start":5,"highlight_end":16}],"label":"no `local_waker` in `task`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/futures-api.rs","byte_start":258,"byte_end":283,"line_start":13,"line_end":13,"column_start":18,"column_end":43,"is_primary":true,"text":[{"text":"    local_waker, local_waker_from_nonlocal,","highlight_start":18,"highlight_end":43}],"label":"no `local_waker_from_nonlocal` in `task`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved imports `std::task::Wake`, `std::task::LocalWaker`, `std::task::local_waker`, `std::task::local_waker_from_nonlocal`\n  --> /checkout/src/test/run-pass/futures-api.rs:12:11\n   |\nLL |     Poll, Wake, Waker, LocalWaker,\n   |           ^^^^         ^^^^^^^^^^ no `LocalWaker` in `task`\n   |           |\n   |           no `Wake` in `task`. Did you mean to use `Waker`?\nLL |     local_waker, local_waker_from_nonlocal,\n   |     ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^ no `local_waker_from_nonlocal` in `task`\n   |     |\n   |     no `local_waker` in `task`\n\n"}
[01:15:39] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:15:39] 
[01:15:39] ------------------------------------------
[01:15:39] 
---
[01:15:39] 
[01:15:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:15:39] 
[01:15:39] 
[01:15:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:39] 
[01:15:39] 
[01:15:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:39] Build completed unsuccessfully in 0:11:02
[01:15:39] Build completed unsuccessfully in 0:11:02
[01:15:39] make: *** [check] Error 1
[01:15:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00870028
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 30 10:55:31 UTC 2019
---
travis_time:end:173df634:start=1548845733247831665,finish=1548845733252722135,duration=4890470
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ea2231b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
