\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs","byte_start":8357,"byte_end":8358,"line_start":211,"line_end":211,"column_start":35,"column_end":36,"is_primary":true,"text":[{"text":"        AddParens.visit_expr(&mut e);","highlight_start":35,"highlight_end":36}],"label":"cannot borrow mutably","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"make this binding mutable","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs","byte_start":7452,"byte_end":7453,"line_start":196,"line_end":196,"column_start":25,"column_end":26,"is_primary":true,"text":[{"text":"    iter_exprs(2, &mut |e| {","highlight_start":25,"highlight_end":26}],"label":null,"suggested_replacement":"mut e","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable argument `e` as mutable\n  --> /checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs:211:35\n   |\nLL |     iter_exprs(2, &mut |e| {\n   |                         - help: make this binding mutable: `mut e`\n...\nLL |         AddParens.visit_expr(&mut e);\n   |                                   ^ cannot borrow mutably\n\n"}
[01:29:18] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[01:29:18] 
[01:29:18] ------------------------------------------
[01:29:18] 
---
[01:29:18] test result: FAILED. 46 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:29:18] 
[01:29:18] 
[01:29:18] 
[01:29:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:29:18] 
[01:29:18] 
[01:29:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:18] Build completed unsuccessfully in 0:17:35
[01:29:18] Build completed unsuccessfully in 0:17:35
[01:29:18] Makefile:48: recipe for target 'check' failed
[01:29:18] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01085658
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 15:27:11 UTC 2019
---
travis_time:end:0061edba:start=1549466833503414403,finish=1549466833508735099,duration=5320696
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:112f8ec0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11e7dd40
travis_time:start:11e7dd40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cc3fbd9
$ dmesg | grep -i kill
