\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/try-block/try-block-bad-type.rs","byte_start":479,"byte_end":484,"line_start":19,"line_end":19,"column_start":24,"column_end":29,"is_primary":true,"text":[{"text":"    let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied","highlight_start":24,"highlight_end":29}],"label":"the trait `std::ops::Try` is not implemented for `i32`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/try-block/try-block-bad-type.rs","byte_start":479,"byte_end":484,"line_start":19,"line_end":19,"column_start":24,"column_end":29,"is_primary":false,"text":[{"text":"    let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied","highlight_start":24,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `try block`","def_site_span":{"file_name":"/checkout/src/test/ui/try-block/try-block-bad-type.rs","byte_start":479,"byte_end":484,"line_start":19,"line_end":19,"column_start":24,"column_end":29,"is_primary":false,"text":[{"text":"    let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied","highlight_start":24,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"required by `std::ops::Try::from_ok`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `i32: std::ops::Try` is not satisfied\n  --> /checkout/src/test/ui/try-block/try-block-bad-type.rs:19:24\n   |\nLL |     let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied\n   |                        ^^^^^ the trait `std::ops::Try` is not implemented for `i32`\n   |\n   = note: required by `std::ops::Try::from_ok`\n\n"}
[01:05:32] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:05:32] {"message":"Some errors occurred: E0271, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0271, E0277.\n"}
[01:05:32] 
[01:05:32] ------------------------------------------
[01:05:32] 
[01:05:32] thread '[ui] ui/try-block/try-block-bad-type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:05:32] 
[01:05:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:05:32] 
[01:05:32] 
[01:05:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:32] 
[01:05:32] 
[01:05:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:32] Build completed unsuccessfully in 0:04:17
[01:05:32] Build completed unsuccessfully in 0:04:17
[01:05:32] Makefile:48: recipe for target 'check' failed
[01:05:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:135b6d7e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 04:32:55 UTC 2019
---
travis_time:end:0e060fd0:start=1554870776403805454,finish=1554870776408359803,duration=4554349
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a3dbe80
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08b5036c
travis_time:start:08b5036c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00001728
$ dmesg | grep -i kill
