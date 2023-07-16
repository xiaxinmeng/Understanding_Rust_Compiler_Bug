\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs","byte_start":1332,"byte_end":1337,"line_start":42,"line_end":42,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let Ok(x) = x;","highlight_start":9,"highlight_end":14}],"label":"pattern `Err(_)` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0005]: refutable pattern in local binding: `Err(_)` not covered\n  --> /checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs:42:9\n   |\nLL |     let Ok(x) = x;\n   |         ^^^^^ pattern `Err(_)` not covered\n\n"}
[00:53:48] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:53:48] {"message":"Some errors occurred: E0004, E0005.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0004, E0005.\n"}
[00:53:48] {"message":"For more information about an error, try `rustc --explain E0004`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0004`.\n"}
[00:53:48] ------------------------------------------
[00:53:48] 
[00:53:48] thread '[ui] ui/uninhabited/uninhabited-matches-feature-gated.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:53:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:53:48] 
[00:53:48] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:53:48] 
[00:53:48] 
[00:53:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:48] 
[00:53:48] 
[00:53:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:48] Build completed unsuccessfully in 0:08:03
[00:53:48] Build completed unsuccessfully in 0:08:03
[00:53:48] Makefile:58: recipe for target 'check' failed
[00:53:48] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:031817e1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0179bdce:start=1536670857442547996,finish=1536670857451384567,duration=8836571
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a420044
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17b764fc
travis_time:start:17b764fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0359db09
$ dmesg | grep -i kill
