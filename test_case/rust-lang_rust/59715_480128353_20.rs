\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":5284,"byte_end":5285,"line_start":159,"line_end":159,"column_start":7,"column_end":8,"is_primary":true,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":7,"highlight_end":8}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":5282,"byte_end":5283,"line_start":159,"line_end":159,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":5,"highlight_end":6}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":5282,"byte_end":5289,"line_start":159,"line_end":159,"column_start":5,"column_end":12,"is_primary":false,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":5,"highlight_end":12}],"label":"mutable borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable\n  --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:159:7\n   |\nLL |     i[i[3]] = i[4];\n   |     --^----\n   |     | |\n   |     | immutable borrow occurs here\n   |     mutable borrow occurs here\n   |     mutable borrow later used here\n\n"}
[01:31:30] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[01:31:30] {"message":"Some errors occurred: E0382, E0499, E0502.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0499, E0502.\n"}
[01:31:30] 
[01:31:30] ------------------------------------------
[01:31:30] 
[01:31:30] thread '[ui (nll)] ui/borrowck/two-phase-nonrecv-autoref.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:31:30] 
[01:31:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:31:30] 
[01:31:30] 
[01:31:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:31:30] 
[01:31:30] 
[01:31:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:30] Build completed unsuccessfully in 0:08:45
[01:31:30] Build completed unsuccessfully in 0:08:45
[01:31:30] Makefile:48: recipe for target 'check' failed
[01:31:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:009f8872
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr  5 02:31:28 UTC 2019
---
travis_time:end:09d92ff4:start=1554431490325654067,finish=1554431490341937118,duration=16283051
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13242bff
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bc95958
travis_time:start:1bc95958
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2bfb856c
$ dmesg | grep -i kill
