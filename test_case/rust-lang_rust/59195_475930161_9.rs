\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/ch04-00-understanding-ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/suggestions/borrow-for-loop-head.rs","byte_start":74,"byte_end":75,"line_start":4,"line_end":4,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"        for j in a {","highlight_start":18,"highlight_end":19}],"label":"value moved here, in previous iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/suggestions/borrow-for-loop-head.rs","byte_start":74,"byte_end":75,"line_start":4,"line_end":4,"column_start":18,"column_end":19,"is_primary":false,"text":[{"text":"        for j in a {","highlight_start":18,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"/checkout/src/test/ui/suggestions/borrow-for-loop-head.rs","byte_start":74,"byte_end":75,"line_start":4,"line_end":4,"column_start":18,"column_end":19,"is_primary":false,"text":[{"text":"        for j in a {","highlight_start":18,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/suggestions/borrow-for-loop-head.rs","byte_start":20,"byte_end":21,"line_start":2,"line_end":2,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"    let a = vec![1, 2, 3];","highlight_start":9,"highlight_end":10}],"label":"move occurs because `a` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0382]: use of moved value: `a`\n  --> /checkout/src/test/ui/suggestions/borrow-for-loop-head.rs:4:18\n   |\nLL |     let a = vec![1, 2, 3];\n   |         - move occurs because `a` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait\nLL |     for i in &a {\nLL |         for j in a {\n   |                  ^ value moved here, in previous iteration of loop\n\n"}
[01:40:36] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:40:36] {"message":"Some errors occurred: E0382, E0505.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0505.\n"}
[01:40:36] 
[01:40:36] ------------------------------------------
[01:40:36] 
[01:40:36] thread '[ui (nll)] ui/suggestions/borrow-for-loop-head.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
---
[01:40:36] 
[01:40:36] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:40:36] 
[01:40:36] 
[01:40:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:40:36] 
[01:40:36] 
[01:40:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:36] Build completed unsuccessfully in 0:08:08
[01:40:36] Build completed unsuccessfully in 0:08:08
[01:40:36] Makefile:48: recipe for target 'check' failed
[01:40:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05bd9ca0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 24 05:36:21 UTC 2019
---
travis_time:end:00f079cd:start=1553405783500252427,finish=1553405783516598457,duration=16346030
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ea09360
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22b4d2b2
travis_time:start:22b4d2b2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:313edb69
$ dmesg | grep -i kill
