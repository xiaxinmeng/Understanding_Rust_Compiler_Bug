\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/levenshtein.rs","byte_start":624,"byte_end":630,"line_start":28,"line_end":28,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"    let b: m::first = m::second; // Misspelled item in module.","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a unit struct with a similar name exists","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/resolve/levenshtein.rs","byte_start":624,"byte_end":630,"line_start":28,"line_end":28,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"    let b: m::first = m::second; // Misspelled item in module.","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":"Second","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0425]: cannot find value `second` in module `m`\n  --> /checkout/src/test/ui/resolve/levenshtein.rs:28:26\n   |\nLL |     let b: m::first = m::second; // Misspelled item in module.\n   |                          ^^^^^^ help: a unit struct with a similar name exists: `Second`\n\n"}
[01:15:56] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[01:15:56] {"message":"Some errors occurred: E0412, E0425.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0412, E0425.\n"}
[01:15:56] 
[01:15:56] ------------------------------------------
[01:15:56] 
[01:15:56] thread '[ui] ui/resolve/levenshtein.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
---
[01:15:56] 
[01:15:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:15:56] 
[01:15:56] 
[01:15:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:56] 
[01:15:56] 
[01:15:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:56] Build completed unsuccessfully in 0:04:35
[01:15:56] Build completed unsuccessfully in 0:04:35
[01:15:56] Makefile:48: recipe for target 'check' failed
[01:15:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:098c1000
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 18:35:40 UTC 2019
---
travis_time:end:0484e65d:start=1553538942719020064,finish=1553538942724524339,duration=5504275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2c2b643e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:018c4113
travis_time:start:018c4113
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:198e6922
$ dmesg | grep -i kill
