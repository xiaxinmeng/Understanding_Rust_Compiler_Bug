\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs","byte_start":56,"byte_end":81,"line_start":4,"line_end":4,"column_start":20,"column_end":45,"is_primary":true,"text":[{"text":"    let _: usize = unclosed_delim_mod::new();","highlight_start":20,"highlight_end":45}],"label":"expected usize, found enum `std::result::Result`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `usize`\n   found type `std::result::Result<unclosed_delim_mod::Value, ()>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs:4:20\n   |\nLL |     let _: usize = unclosed_delim_mod::new();\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected usize, found enum `std::result::Result`\n   |\n   = note: expected type `usize`\n              found type `std::result::Result<unclosed_delim_mod::Value, ()>`\n\n"}
[01:08:27] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] 
---
[01:08:27] 
[01:08:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:08:27] 
[01:08:27] 
[01:08:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:27] 
[01:08:27] 
[01:08:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:27] Build completed unsuccessfully in 0:04:07
[01:08:27] Build completed unsuccessfully in 0:04:07
[01:08:27] make: *** [check] Error 1
[01:08:27] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13eead92
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar  4 02:10:48 UTC 2019
---
travis_time:end:05a36868:start=1551665449779969625,finish=1551665449784415889,duration=4446264
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0134459c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08f3b79d
travis_time:start:08f3b79d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:116be940
$ dmesg | grep -i kill
