\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs","byte_start":109,"byte_end":141,"line_start":5,"line_end":5,"column_start":45,"column_end":77,"is_primary":true,"text":[{"text":"pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };","highlight_start":45,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0019]: static contains unimplemented expression type\n  --> /checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs:5:45\n   |\nLL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };\n   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:26] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:04:26] {"message":"Some errors occurred: E0017, E0019.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0017, E0019.\n"}
[01:04:26] {"message":"For more information about an error, try `rustc --explain E0017`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0017`.\n"}
[01:04:26] ------------------------------------------
[01:04:26] 
[01:04:26] thread '[ui] ui/consts/static_mut_containing_mut_ref2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:04:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:04:26] 
[01:04:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:04:26] 
[01:04:26] 
[01:04:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:26] 
[01:04:26] 
[01:04:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:26] Build completed unsuccessfully in 0:03:56
[01:04:26] Build completed unsuccessfully in 0:03:56
[01:04:26] Makefile:58: recipe for target 'check' failed
[01:04:26] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26b854ea
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 20 12:31:02 UTC 2018
---
travis_time:end:014e205a:start=1545309063187079607,finish=1545309063191509882,duration=4430275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:117c39dd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0453c6cc
travis_time:start:0453c6cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11b1bc44
$ dmesg | grep -i kill
