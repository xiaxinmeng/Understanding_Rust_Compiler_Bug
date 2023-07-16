\n\nSee the 'Use Declarations' section of the reference for more information on\nthis topic:\n\nhttps://doc.rust-lang.org/reference.html#use-declarations\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/macro-rules.rs","byte_start":247,"byte_end":264,"line_start":11,"line_end":11,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"    pub use legacy_macro as _; //~ ERROR `legacy_macro` is private, and cannot be re-exported","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider marking `legacy_macro` as `pub` in the imported module","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/macro-rules.rs","byte_start":247,"byte_end":264,"line_start":11,"line_end":11,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"    pub use legacy_macro as _; //~ ERROR `legacy_macro` is private, and cannot be re-exported","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0364]: `legacy_macro` is private, and cannot be re-exported\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/macro-rules.rs:11:13\n   |\nLL |     pub use legacy_macro as _; //~ ERROR `legacy_macro` is private, and cannot be re-exported\n   |             ^^^^^^^^^^^^^^^^^\n   |\nnote: consider marking `legacy_macro` as `pub` in the imported module\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/macro-rules.rs:11:13\n   |\nLL |     pub use legacy_macro as _; //~ ERROR `legacy_macro` is private, and cannot be re-exported\n   |             ^^^^^^^^^^^^^^^^^\n\n"}
[01:11:32] thread 'rustc' panicked at 'assertion failed: !name.is_gensymed()', src/libsyntax/feature_gate.rs:1330:1
[01:11:32] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:11:32] {"message":"For more information about this error, try `rustc --explain E0364`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0364`.\n"}
[01:11:32] 
[01:11:32] error: internal compiler error: unexpected panic
[01:11:32] error: internal compiler error: unexpected panic
[01:11:32] 
[01:11:32] note: the compiler unexpectedly panicked. this is a bug.
[01:11:32] 
[01:11:32] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:32] 
[01:11:32] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:32] 
[01:11:32] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:11:32] 
[01:11:32] ------------------------------------------
[01:11:32] 
[01:11:32] thread '[ui] ui/rust-2018/uniform-paths/macro-rules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:11:32] 
[01:11:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:11:32] 
[01:11:32] 
[01:11:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:32] 
[01:11:32] 
[01:11:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:32] Build completed unsuccessfully in 0:05:00
[01:11:32] Build completed unsuccessfully in 0:05:00
[01:11:32] Makefile:48: recipe for target 'check' failed
[01:11:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02c709c2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 11 06:02:36 UTC 2019
---
travis_time:end:0fa307a8:start=1554962558757463843,finish=1554962558763587259,duration=6123416
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:013d01a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09aaef89
travis_time:start:09aaef89
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0096375f
$ dmesg | grep -i kill
