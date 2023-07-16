\n\nCheck how many fields the enum was declared with and ensure that your pattern\nuses the same number.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs","byte_start":1933,"byte_end":1948,"line_start":49,"line_end":49,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"        Failure(_, tok) => {","highlight_start":9,"highlight_end":24}],"label":"expected 3 fields, found 2","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:49:9\n   |\nLL |         Failure(_, tok) => {\n   |         ^^^^^^^^^^^^^^^ expected 3 fields, found 2\n\n"}
[01:10:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:10:30] {"message":"For more information about this error, try `rustc --explain E0023`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0023`.\n"}
[01:10:30] ------------------------------------------
[01:10:30] 
[01:10:30] thread '[run-pass] run-pass-fulldeps/mbe_matching_test_macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:10:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:10:30] test result: FAILED. 96 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:10:30] 
[01:10:30] 
[01:10:30] 
[01:10:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:30] 
[01:10:30] 
[01:10:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:30] Build completed unsuccessfully in 0:24:34
[01:10:30] Build completed unsuccessfully in 0:24:34
[01:10:30] Makefile:58: recipe for target 'check' failed
[01:10:30] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1da1ed18
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1b5e2694:start=1541453933488531773,finish=1541453933664584221,duration=176052448
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04bf213f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12ae6d0d
$ dmesg | grep -i kill
