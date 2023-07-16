\n\nNote that empty arrays `[T; 0]` have the same alignment requirement as the\nelement type `T`. Also note that the error is conservatively reported even when\nthe alignment of the zero-sized type is less than or equal to the data field's\nalignment.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/repr/repr-transparent.rs","byte_start":1723,"byte_end":1736,"line_start":50,"line_end":50,"column_start":24,"column_end":37,"is_primary":true,"text":[{"text":"struct GenericAlign<T>(ZstAlign32<T>, u32); //~ ERROR alignment larger than 1","highlight_start":24,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0691]: zero-sized field in transparent struct has alignment larger than 1\n  --> /checkout/src/test/ui/repr/repr-transparent.rs:50:24\n   |\nLL | struct GenericAlign<T>(ZstAlign32<T>, u32); //~ ERROR alignment larger than 1\n   |                        ^^^^^^^^^^^^^\n\n"}
[00:49:35] {"message":"zero-sized field in transparent struct has alignment larger than 1","code":{"code":"E0691","e try `rustc --explain E0690`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0690`.\n"}
[00:49:35] ------------------------------------------
[00:49:35] 
[00:49:35] thread '[ui] ui/repr/repr-transparent.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:49:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:35] 
[00:49:35] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:35] 
[00:49:35] 
[00:49:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:35] 
[00:49:35] 
[00:49:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:35] Build completed unsuccessfully in 0:03:37
[00:49:35] Build completed unsuccessfully in 0:03:37
[00:49:35] make: *** [check] Error 1
[00:49:35] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d47fac0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02789484:start=1541198047972089228,finish=1541198047977509434,duration=5420206
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:092d79d6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!check
