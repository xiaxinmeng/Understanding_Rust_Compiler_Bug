\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-35677.rs","byte_start":116,"byte_end":128,"line_start":3,"line_end":3,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    this.drain()","highlight_start":5,"highlight_end":17}],"label":"expected bool, found struct `std::collections::hash_map::Drain`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-35677.rs","byte_start":105,"byte_end":109,"line_start":2,"line_end":2,"column_start":75,"column_end":79,"is_primary":false,"text":[{"text":"fn intersect_map<K, V>(this: &mut HashMap<K, V>, other: HashMap<K, V>) -> bool {","highlight_start":75,"highlight_end":79}],"label":"expected `bool` because of return type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `bool`\n   found type `std::collections::hash_map::Drain<'_, K, V>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/issues/issue-35677.rs:3:5\n   |\nLL | fn intersect_map<K, V>(this: &mut HashMap<K, V>, other: HashMap<K, V>) -> bool {\n   |                                                                           ---- expected `bool` because of return type\nLL |     this.drain()\n   |     ^^^^^^^^^^^^ expected bool, found struct `std::collections::hash_map::Drain`\n   |\n   = note: expected type `bool`\n              found type `std::collections::hash_map::Drain<'_, K, V>`\n\n"}
[01:02:06] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:02:06] 
[01:02:06] ------------------------------------------
[01:02:06] 
---
[01:02:06] 
[01:02:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:02:06] 
[01:02:06] 
[01:02:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:06] 
[01:02:06] 
[01:02:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:06] Build completed unsuccessfully in 0:04:27
[01:02:06] Build completed unsuccessfully in 0:04:27
[01:02:06] make: *** [check] Error 1
[01:02:06] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:080e3d1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 12:21:14 UTC 2019
---
travis_time:end:21df20b7:start=1549887675931567109,finish=1549887675936415104,duration=4847995
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2638dd9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!che
