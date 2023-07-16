\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-18118.rs","byte_start":75,"byte_end":77,"line_start":4,"line_end":4,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        &p //~ ERROR `p` does not live long enough","highlight_start":9,"highlight_end":11}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-18118.rs","byte_start":122,"byte_end":123,"line_start":5,"line_end":5,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    };","highlight_start":5,"highlight_end":6}],"label":"`p` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-18118.rs","byte_start":75,"byte_end":77,"line_start":4,"line_end":4,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        &p //~ ERROR `p` does not live long enough","highlight_start":9,"highlight_end":11}],"label":"using this value as a constant requires that `p` is borrowed for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `p` does not live long enough\n  --> /checkout/src/test/ui/issues/issue-18118.rs:4:9\n   |\nLL |         &p //~ ERROR `p` does not live long enough\n   |         ^^\n   |         |\n   |         borrowed value does not live long enough\n   |         using this value as a constant requires that `p` is borrowed for `'static`\nLL |     };\n   |     - `p` dropped here while still borrowed\n\n"}
[01:31:45] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[01:31:45] 
[01:31:45] ------------------------------------------
[01:31:45] 
---
[01:31:45] test result: FAILED. 5206 passed; 4 failed; 88 ignored; 0 measured; 0 filtered out
[01:31:45] 
[01:31:45] 
[01:31:45] 
[01:31:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:31:45] 
[01:31:45] 
[01:31:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:45] Build completed unsuccessfully in 0:08:13
[01:31:45] Build completed unsuccessfully in 0:08:13
[01:31:45] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cd3f1ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 11 16:26:26 UTC 2019
---
travis_time:end:1568155e:start=1547223988757316470,finish=1547223988774942874,duration=17626404
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0288027c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c639693
travis_time:start:0c639693
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06bf00ee
$ dmesg | grep -i kill
