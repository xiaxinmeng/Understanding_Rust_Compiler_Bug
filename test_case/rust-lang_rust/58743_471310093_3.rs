\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-26619.rs","byte_start":200,"byte_end":221,"line_start":7,"line_end":7,"column_start":76,"column_end":97,"is_primary":true,"text":[{"text":"        for s in vec![\"1|2\".to_string()].into_iter().filter_map(|ref line| self.make_entry(line)) {","highlight_start":76,"highlight_end":97}],"label":"returns a value referencing data owned by the current function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-26619.rs","byte_start":190,"byte_end":198,"line_start":7,"line_end":7,"column_start":66,"column_end":74,"is_primary":false,"text":[{"text":"        for s in vec![\"1|2\".to_string()].into_iter().filter_map(|ref line| self.make_entry(line)) {","highlight_start":66,"highlight_end":74}],"label":"function parameter borrowed here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0515]: cannot return value referencing function parameter\n  --> /checkout/src/test/ui/issues/issue-26619.rs:7:76\n   |\nLL |         for s in vec![\"1|2\".to_string()].into_iter().filter_map(|ref line| self.make_entry(line)) {\n   |                                                                  --------  ^^^^^^^^^^^^^^^^^^^^^ returns a value referencing data owned by the current function\n   |                                                                  |\n   |                                                                  function parameter borrowed here\n\n"}
[01:46:55] {"message":"For more information about this error, try `rustc --explain E0515`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0515`.\n"}
[01:46:55] 
[01:46:55] ------------------------------------------
[01:46:55] 
---
[01:46:55] 
[01:46:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:46:55] 
[01:46:55] 
[01:46:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:46:55] 
[01:46:55] 
[01:46:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:55] Build completed unsuccessfully in 0:09:00
[01:46:55] Build completed unsuccessfully in 0:09:00
[01:46:55] Makefile:48: recipe for target 'check' failed
[01:46:55] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bf78654
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 10 14:14:53 UTC 2019
---
travis_time:end:00f12e3e:start=1552227295089952393,finish=1552227295104529173,duration=14576780
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0654fefa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08d2ac1e
travis_time:start:08d2ac1e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14fded5b
$ dmesg | grep -i kill
