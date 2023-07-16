\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs","byte_start":5048,"byte_end":5049,"line_start":172,"line_end":172,"column_start":24,"column_end":25,"is_primary":true,"text":[{"text":"    let _seetype: () = z; //~ ERROR mismatched types","highlight_start":24,"highlight_end":25}],"label":"expected (), found u32","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `()`\n   found type `u32`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:172:24\n   |\nLL |     let _seetype: () = z; //~ ERROR mismatched types\n   |                        ^ expected (), found u32\n   |\n   = note: expected type `()`\n              found type `u32`\n\n"}
[01:08:54] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[01:08:54] {"message":"Some errors occurred: E0034, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0034, E0308.\n"}
[01:08:54] 
[01:08:54] ------------------------------------------
[01:08:54] 
[01:08:54] thread '[ui] ui/methods/method-deref-to-same-trait-object-with-separate-params.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3287:9
---
[01:08:54] 
[01:08:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:08:54] 
[01:08:54] 
[01:08:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:54] 
[01:08:54] 
[01:08:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:54] Build completed unsuccessfully in 0:04:24
[01:08:54] Build completed unsuccessfully in 0:04:24
[01:08:54] Makefile:48: recipe for target 'check' failed
[01:08:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12999b00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 26 23:10:06 UTC 2019
---
travis_time:end:128aec30:start=1548544207906249189,finish=1548544207911977778,duration=5728589
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03ec39d6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!chec
