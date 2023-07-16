\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs","byte_start":1163,"byte_end":1172,"line_start":32,"line_end":32,"column_start":16,"column_end":25,"is_primary":true,"text":[{"text":"        ty!()::AssocItem => {}","highlight_start":16,"highlight_end":25}],"label":"associated item not found in `u8`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no associated item named `AssocItem` found for type `u8` in the current scope\n  --> /checkout/src/test/ui/did_you_mean/bad-assoc-pat.rs:32:16\n   |\nLL |         ty!()::AssocItem => {}\n   |                ^^^^^^^^^ associated item not found in `u8`\n\n"}
[01:15:51] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:15:51] 
[01:15:51] ------------------------------------------
[01:15:51] 
---
[01:15:51] 
[01:15:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:15:51] 
[01:15:51] 
[01:15:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:51] 
[01:15:51] 
[01:15:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:51] Build completed unsuccessfully in 0:04:49
[01:15:51] Build completed unsuccessfully in 0:04:49
[01:15:51] make: *** [check] Error 1
[01:15:51] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:014741c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 22 14:12:06 UTC 2019
---
travis_time:end:26de7ed3:start=1553263927680427904,finish=1553263927685737989,duration=5310085
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2c247f87
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0602150c
travis_time:start:0602150c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ede1e03
$ dmesg | grep -i kill
