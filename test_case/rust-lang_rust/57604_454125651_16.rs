\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/str/str-mut-idx.rs","byte_start":166,"byte_end":175,"line_start":7,"line_end":7,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    s[1usize] = bot();","highlight_start":5,"highlight_end":14}],"label":"slice indices are of type `usize` or ranges of `usize`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::slice::SliceIndex<str>` is not implemented for `usize`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `std::ops::Index<usize>` for `str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `usize: std::slice::SliceIndex<str>` is not satisfied\n  --> /checkout/src/test/ui/str/str-mut-idx.rs:7:5\n   |\nLL |     s[1usize] = bot();\n   |     ^^^^^^^^^ slice indices are of type `usize` or ranges of `usize`\n   |\n   = help: the trait `std::slice::SliceIndex<str>` is not implemented for `usize`\n   = note: required because of the requirements on the impl of `std::ops::Index<usize>` for `str`\n\n"}
[01:06:10] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:06:10] 
[01:06:10] ------------------------------------------
[01:06:10] 
[01:06:10] thread '[ui] ui/str/str-mut-idx.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:06:10] 
[01:06:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:06:10] 
[01:06:10] 
[01:06:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:10] 
[01:06:10] 
[01:06:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:10] Build completed unsuccessfully in 0:04:10
[01:06:10] Build completed unsuccessfully in 0:04:10
[01:06:10] Makefile:48: recipe for target 'check' failed
[01:06:10] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04a7b5d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 14 19:13:53 UTC 2019
---
travis_time:end:1eaae0c8:start=1547493234157595398,finish=1547493234162237605,duration=4642207
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:313a8d84
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "
