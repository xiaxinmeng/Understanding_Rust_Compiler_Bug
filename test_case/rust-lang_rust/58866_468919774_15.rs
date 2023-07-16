\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-ptr-unique.rs","byte_start":167,"byte_end":184,"line_start":8,"line_end":8,"column_start":33,"column_end":50,"is_primary":true,"text":[{"text":"    let x: &'static *mut u32 = &(unique.as_ptr());","highlight_start":33,"highlight_end":50}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-unique.rs","byte_start":242,"byte_end":243,"line_start":10,"line_end":10,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-unique.rs","byte_start":146,"byte_end":163,"line_start":8,"line_end":8,"column_start":12,"column_end":29,"is_primary":false,"text":[{"text":"    let x: &'static *mut u32 = &(unique.as_ptr());","highlight_start":12,"highlight_end":29}],"label":"type annotation requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-ptr-unique.rs:8:33\n   |\nLL |     let x: &'static *mut u32 = &(unique.as_ptr());\n   |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\n   |            |\n   |            type annotation requires that borrow lasts for `'static`\nLL |     //~^ ERROR borrowed value does not live long enough\nLL | }\n   | - temporary value is freed at the end of this statement\n\n"}
[01:34:01] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:34:01] 
[01:34:01] ------------------------------------------
[01:34:01] 
---
[01:34:01] 
[01:34:01] 
[01:34:01] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:34:01] 
[01:34:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:34:01] 
[01:34:01] 
[01:34:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:01] Build completed unsuccessfully in 0:08:11
[01:34:01] Build completed unsuccessfully in 0:08:11
[01:34:01] Makefile:48: recipe for target 'check' failed
[01:34:01] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ae29455
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  2 13:14:24 UTC 2019
---
travis_time:end:0f6a8ec4:start=1551532466167932283,finish=1551532466185903182,duration=17970899
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05e6c7d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1407f750
travis_time:start:1407f750
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:30c7d21e
$ dmesg | grep -i kill
