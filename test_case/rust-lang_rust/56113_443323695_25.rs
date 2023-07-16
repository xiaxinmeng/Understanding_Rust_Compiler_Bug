\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/vec/vec-mut-iter-borrow.rs","byte_start":530,"byte_end":537,"line_start":14,"line_end":14,"column_start":14,"column_end":21,"is_primary":false,"text":[{"text":"    for x in &mut xs {","highlight_start":14,"highlight_end":21}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/vec/vec-mut-iter-borrow.rs","byte_start":548,"byte_end":550,"line_start":15,"line_end":15,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        xs.push(1) //~ ERROR cannot borrow `xs`","highlight_start":9,"highlight_end":11}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/vec/vec-mut-iter-borrow.rs","byte_start":530,"byte_end":537,"line_start":14,"line_end":14,"column_start":14,"column_end":21,"is_primary":false,"text":[{"text":"    for x in &mut xs {","highlight_start":14,"highlight_end":21}],"label":"first borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `xs` as mutable more than once at a time\n  --> /checkout/src/test/ui/vec/vec-mut-iter-borrow.rs:15:9\n   |\nLL |     for x in &mut xs {\n   |              -------\n   |              |\n   |              first mutable borrow occurs here\n   |              first borrow later used here\nLL |         xs.push(1) //~ ERROR cannot borrow `xs`\n   |         ^^ second mutable borrow occurs here\n\n"}
[00:56:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:19] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] thread '[ui (nll)] ui/vec/vec-mut-iter-borrow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:56:19] 
---
[00:56:19] 
[00:56:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:56:19] 
[00:56:19] 
[00:56:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:56:19] 
[00:56:19] 
[00:56:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:19] Build completed unsuccessfully in 0:06:54
[00:56:19] Build completed unsuccessfully in 0:06:54
[00:56:19] make: *** [check] Error 1
[00:56:19] Makefile:58: recipe for target 'check' failed
 [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15ebcbec
travis_time:start:15ebcbec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:037f7040
$ dmesg | grep -i kill
