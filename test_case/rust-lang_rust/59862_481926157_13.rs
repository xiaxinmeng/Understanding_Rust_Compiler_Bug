\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/gated-plugin.rs","byte_start":34,"byte_end":62,"line_start":3,"line_end":3,"column_start":1,"column_end":29,"is_primary":true,"text":[{"text":"#![plugin(attr_plugin_test)]","highlight_start":1,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see tracking issue #29597","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add #![feature(plugin)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: compiler plugins are experimental and possibly buggy\n  --> /checkout/src/test/ui-fulldeps/gated-plugin.rs:3:1\n   |\nLL | #![plugin(attr_plugin_test)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: for more information, see tracking issue #29597\n   = help: add #![feature(plugin)] to the crate attributes to enable\n\n"}
[01:19:05] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:19:05] 
[01:19:05] ------------------------------------------
[01:19:05] 
---
[01:19:05] test result: FAILED. 20 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:19:05] 
[01:19:05] 
[01:19:05] 
[01:19:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:05] 
[01:19:05] 
[01:19:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:05] Build completed unsuccessfully in 0:13:03
[01:19:05] Build completed unsuccessfully in 0:13:03
[01:19:05] make: *** [check] Error 1
[01:19:05] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01da16e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 11 01:02:36 UTC 2019
---
travis_time:end:0139fec6:start=1554944558180746733,finish=1554944558185729668,duration=4982935
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b29d648
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ecdf3e3
travis_time:start:1ecdf3e3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1fee26ec
$ dmesg | grep -i kill
