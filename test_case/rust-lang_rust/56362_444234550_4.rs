\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/precise_pointer_size_matching.rs","byte_start":402,"byte_end":408,"line_start":22,"line_end":22,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"    match 0usize { //~ ERROR non-exhaustive patterns","highlight_start":11,"highlight_end":17}],"label":"patterns `0usize` and `21usize..=4294967295usize` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0004]: non-exhaustive patterns: `0usize` and `21usize..=4294967295usize` not covered\n  --> /checkout/src/test/ui/precise_pointer_size_matching.rs:22:11\n   |\nLL |     match 0usize { //~ ERROR non-exhaustive patterns\n   |           ^^^^^^ patterns `0usize` and `21usize..=4294967295usize` not covered\n\n"}
[00:44:45] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:44:45] {"message":"For more information about this error, try `rustc --explain E0004`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0004`.\n"}
[00:44:45] ------------------------------------------
[00:44:45] 
[00:44:45] thread '[ui] ui/precise_pointer_size_matching.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:44:45] 
---
[00:44:45] 
[00:44:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:44:45] 
[00:44:45] 
[00:44:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:45] 
[00:44:45] 
[00:44:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:44:45] Build completed unsuccessfully in 0:42:26
---
travis_time:end:048ac034:start=1543953009747468270,finish=1543953009754280267,duration=6811997
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1faa9f00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a5e1a14
travis_time:start:0a5e1a14
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2723e9a4
$ dmesg | grep -i kill
