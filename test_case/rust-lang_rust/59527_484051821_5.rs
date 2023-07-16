\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/unsized-locals/unsized-index.rs","byte_start":531,"byte_end":560,"line_start":17,"line_end":17,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"impl ops::IndexMut<str> for A {","highlight_start":1,"highlight_end":30}],"label":"missing `index_mut` in implementation","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`index_mut` from trait: `fn(&mut Self, Idx) -> &mut <Self as std::ops::Index<Idx>>::Output`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0046]: not all trait items implemented, missing: `index_mut`\n  --> /checkout/src/test/run-pass/unsized-locals/unsized-index.rs:17:1\n   |\nLL | impl ops::IndexMut<str> for A {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `index_mut` in implementation\n   |\n   = note: `index_mut` from trait: `fn(&mut Self, Idx) -> &mut <Self as std::ops::Index<Idx>>::Output`\n\n"}
[01:16:55] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:16:55] {"message":"Some errors occurred: E0046, E0407.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0046, E0407.\n"}
[01:16:55] 
[01:16:55] ------------------------------------------
[01:16:55] 
[01:16:55] 
---
[01:16:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:16:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:55] 
[01:16:55] 
[01:16:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:55] 
[01:16:55] 
[01:16:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:55] Build completed unsuccessfully in 0:11:28
[01:16:55] Build completed unsuccessfully in 0:11:28
[01:16:55] make: *** [check] Error 1
[01:16:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0117c30a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 17 11:54:39 UTC 2019
---
travis_time:end:016dd705:start=1555502081392724865,finish=1555502081397698946,duration=4974081
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ad25719
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26b06c1d
travis_time:start:26b06c1d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0429ef82
$ dmesg | grep -i kill
