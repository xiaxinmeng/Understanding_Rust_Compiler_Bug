\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-48636.rs","byte_start":63,"byte_end":92,"line_start":7,"line_end":7,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    /// The ID of the parent core","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"doc comments must come before what they document, maybe a comment was intended with `//`?","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"missing comma here","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-48636.rs","byte_start":58,"byte_end":58,"line_start":6,"line_end":6,"column_start":10,"column_end":10,"is_primary":true,"text":[{"text":"    x: u8","highlight_start":10,"highlight_end":10}],"label":null,"suggested_replacement":",","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0585]: found a documentation comment that doesn't document anything\n  --> /checkout/src/test/ui/issues/issue-48636.rs:7:5\n   |\nLL |     x: u8\n   |          - help: missing comma here: `,`\nLL |     /// The ID of the parent core\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: doc comments must come before what they document, maybe a comment was intended with `//`?\n\n"}
[01:07:13] {"message":"For more information about this error, try `rustc --explain E0585`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0585`.\n"}
[01:07:13] 
[01:07:13] ------------------------------------------
[01:07:13] 
---
[01:07:13] 
[01:07:13] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:07:13] 
[01:07:13] 
[01:07:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:13] 
[01:07:13] 
[01:07:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:13] Build completed unsuccessfully in 0:04:46
[01:07:13] Build completed unsuccessfully in 0:04:46
[01:07:13] make: *** [check] Error 1
[01:07:13] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ae73946
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 06:19:08 UTC 2019
---
travis_time:end:24bdf4a0:start=1549779550145507766,finish=1549779550150983375,duration=5475609
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:055c4cb8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:064bb0af
travis_time:start:064bb0af
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b682471
$ dmesg | grep -i kill
