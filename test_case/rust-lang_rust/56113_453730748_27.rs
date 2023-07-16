\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/vec/vec-mut-iter-borrow.rs","byte_start":63,"byte_end":70,"line_start":4,"line_end":4,"column_start":14,"column_end":21,"is_primary":false,"text":[{"text":"    for x in &mut xs {","highlight_start":14,"highlight_end":21}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/vec/vec-mut-iter-borrow.rs","byte_start":81,"byte_end":83,"line_start":5,"line_end":5,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        xs.push(1) //~ ERROR cannot borrow `xs`","highlight_start":9,"highlight_end":11}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/vec/vec-mut-iter-borrow.rs","byte_start":63,"byte_end":70,"line_start":4,"line_end":4,"column_start":14,"column_end":21,"is_primary":false,"text":[{"text":"    for x in &mut xs {","highlight_start":14,"highlight_end":21}],"label":"first borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `xs` as mutable more than once at a time\n  --> /checkout/src/test/ui/vec/vec-mut-iter-borrow.rs:5:9\n   |\nLL |     for x in &mut xs {\n   |              -------\n   |              |\n   |              first mutable borrow occurs here\n   |              first borrow later used here\nLL |         xs.push(1) //~ ERROR cannot borrow `xs`\n   |         ^^ second mutable borrow occurs here\n\n"}
[01:30:33] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] 
---
[01:30:33] 
[01:30:33] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:30:33] 
[01:30:33] 
[01:30:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:30:33] 
[01:30:33] 
[01:30:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:33] Build completed unsuccessfully in 0:08:16
[01:30:33] Build completed unsuccessfully in 0:08:16
[01:30:33] Makefile:48: recipe for target 'check' failed
[01:30:33] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1fdc2884
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 12 08:38:52 UTC 2019
---
travis_time:end:05f30582:start=1547282334260531261,finish=1547282334268438536,duration=7907275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2120d684
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:016dc77b
travis_time:start:016dc77b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01109911
$ dmesg | grep -i kill
