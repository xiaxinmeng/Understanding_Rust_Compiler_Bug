\n"},"level":"error","spans":[{"file_name":"/checkout/src/libcore/hash/mod.rs","byte_start":4730,"byte_end":4731,"line_start":185,"line_end":185,"column_start":13,"column_end":14,"is_primary":false,"text":[{"text":"    fn hash<H: Hasher>(&self, state: &mut H);","highlight_start":13,"highlight_end":14}],"label":"declaration in trait here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":983,"byte_end":994,"line_start":40,"line_end":40,"column_start":33,"column_end":44,"is_primary":true,"text":[{"text":"    fn hash(&self, hasher: &mut impl Hasher) {}","highlight_start":33,"highlight_end":44}],"label":"expected generic parameter, found `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0643]: method `hash` has incompatible signature for trait\n  --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:40:33\n   |\nLL |     fn hash(&self, hasher: &mut impl Hasher) {}\n   |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`\n   | \n  ::: /checkout/src/libcore/hash/mod.rs:185:13\n   |\nLL |     fn hash<H: Hasher>(&self, state: &mut H);\n   |             - declaration in trait here\n\n"}
[00:46:45] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:45] {"message":"For more information about this error, try `rustc --explain E0643`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0643`.\n"}
[00:46:45] ------------------------------------------
[00:46:45] 
[00:46:45] thread '[ui] ui/impl-trait/impl-generic-mismatch.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:45] test result: FAILED. 4169 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
[00:46:45] 
[00:46:45] 
[00:46:45] 
[00:46:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:45] 
[00:46:45] 
[00:46:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:45] Build completed unsuccessfully in 0:03:06
[00:46:45] Build completed unsuccessfully in 0:03:06
[00:46:45] make: *** [check] Error 1
[00:46:45] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00208933
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:066db642:start=1535934881744888500,finish=1535934881751533685,duration=6645185
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:097ec7b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b33d837
travis_time:start:2b33d837
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03f88230
$ dmesg | grep -i kill
