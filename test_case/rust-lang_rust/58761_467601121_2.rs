\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-unwind-attributes.rs","byte_start":285,"byte_end":303,"line_start":11,"line_end":11,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    #[unwind(allowed)] //~ ERROR #[unwind] is experimental","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(unwind_attributes)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: #[unwind] is experimental (see issue #58760)\n  --> /checkout/src/test/ui/feature-gates/feature-gate-unwind-attributes.rs:11:5\n   |\nLL |     #[unwind(allowed)] //~ ERROR #[unwind] is experimental\n   |     ^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(unwind_attributes)] to the crate attributes to enable\n\n"}
[01:04:32] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:04:32] 
[01:04:32] ------------------------------------------
[01:04:32] 
---
[01:04:32] 
[01:04:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:04:32] 
[01:04:32] 
[01:04:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:32] 
[01:04:32] 
[01:04:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:32] Build completed unsuccessfully in 0:04:04
[01:04:32] Build completed unsuccessfully in 0:04:04
[01:04:32] make: *** [check] Error 1
[01:04:32] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:154caac3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 26 20:30:03 UTC 2019
---
travis_time:end:06f46647:start=1551213004902166235,finish=1551213004906540838,duration=4374603
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d2f17d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bdb6298
travis_time:start:0bdb6298
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10529a9c
$ dmesg | grep -i kill
