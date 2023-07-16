\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":6201,"byte_end":6205,"line_start":146,"line_end":146,"column_start":27,"column_end":31,"is_primary":true,"text":[{"text":"const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }","highlight_start":27,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(const_fn)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0723]: function pointers in const fn are unstable (see issue #57563)\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:146:27\n   |\nLL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }\n   |                           ^^^^\n   |\n   = help: add #![feature(const_fn)] to the crate attributes to enable\n\n"}
[01:20:51] {"message":"aborting due to 36 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 36 previous errors\n\n"}
[01:20:51] {"message":"Some errors occurred: E0493, E0515, E0723.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0493, E0515, E0723.\n"}
[01:20:51] 
[01:20:51] ------------------------------------------
[01:20:51] 
[01:20:51] thread '[ui (nll)] ui/consts/min_const_fn/min_const_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:20:51] test result: FAILED. 5293 passed; 2 failed; 88 ignored; 0 measured; 0 filtered out
[01:20:51] 
[01:20:51] 
[01:20:51] 
[01:20:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:20:51] 
[01:20:51] 
[01:20:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:51] Build completed unsuccessfully in 0:07:47
[01:20:51] Build completed unsuccessfully in 0:07:47
[01:20:51] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12d35088
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 03:26:01 UTC 2019
---
travis_time:end:0613fd80:start=1550028362855647495,finish=1550028362866293108,duration=10645613
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d3802c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1952c766
travis_time:start:1952c766
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05c38011
$ dmesg | grep -i kill
