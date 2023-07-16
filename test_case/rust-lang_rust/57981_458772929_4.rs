\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/nested.rs","byte_start":109,"byte_end":114,"line_start":6,"line_end":6,"column_start":29,"column_end":34,"is_primary":true,"text":[{"text":"fn test(_: impl Borrow<impl Debug>) {}","highlight_start":29,"highlight_end":34}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"possible candidate is found in another module, you can import it into scope","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/nested.rs","byte_start":33,"byte_end":33,"line_start":3,"line_end":3,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"use std::borrow::Borrow;","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::fmt::Debug;\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0405]: cannot find trait `Debug` in this scope\n  --> /checkout/src/test/run-pass/impl-trait/nested.rs:6:29\n   |\nLL | fn test(_: impl Borrow<impl Debug>) {}\n   |                             ^^^^^ not found in this scope\nhelp: possible candidate is found in another module, you can import it into scope\n   |\nLL | use std::fmt::Debug;\n   |\n\n"}
[01:13:04] {"message":"unused import: `core::fmt::Debug`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/nested.rs","byte_start":62,"byte_end":78,"line_start":4,"line_end":4,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"use core::fmt::Debug;","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `core::fmt::Debug`\n  --> /checkout/src/test/run-pass/impl-trait/nested.rs:4:5\n   |\nLL | use core::fmt::Debug;\n   |     ^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:13:04] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:13:04] {"message":"Some errors occurred: E0405, E0433, E0666.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0405, E0433, E0666.\n"}
[01:13:04] 
[01:13:04] ------------------------------------------
[01:13:04] 
[01:13:04] thread '[run-pass] run-pass/impl-trait/nested.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3291:9
---
[01:13:04] 
[01:13:04] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:13:04] 
[01:13:04] 
[01:13:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:04] 
[01:13:04] 
[01:13:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:04] Build completed unsuccessfully in 0:10:33
[01:13:04] Build completed unsuccessfully in 0:10:33
[01:13:04] Makefile:48: recipe for target 'check' failed
[01:13:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06e3da80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 30 01:16:30 UTC 2019
---
travis_time:end:072901d1:start=1548810992015867087,finish=1548810992021478033,duration=5610946
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f5500fb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a693c8e
travis_time:start:0a693c8e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1819bb67
$ dmesg | grep -i kill
