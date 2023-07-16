\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/ch04-00-understanding-ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unsized-locals/double-move.rs","byte_start":997,"byte_end":998,"line_start":50,"line_end":50,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"        x.foo();","highlight_start":9,"highlight_end":10}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/unsized-locals/double-move.rs","byte_start":1023,"byte_end":1025,"line_start":51,"line_end":51,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"        let _y = *x; //~ERROR use of moved value","highlight_start":18,"highlight_end":20}],"label":"value used here after move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `*x` has type `str`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `*x`\n  --> /checkout/src/test/ui/unsized-locals/double-move.rs:51:18\n   |\nLL |         x.foo();\n   |         - value moved here\nLL |         let _y = *x; //~ERROR use of moved value\n   |                  ^^ value used here after move\n   |\n   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait\n\n"}
[01:30:22] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] 
---
[01:30:22] test result: FAILED. 5300 passed; 6 failed; 88 ignored; 0 measured; 0 filtered out
[01:30:22] 
[01:30:22] 
[01:30:22] 
[01:30:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:30:22] 
[01:30:22] 
[01:30:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:22] Build completed unsuccessfully in 0:07:51
---
travis_time:end:0a4edff4:start=1550586511476120899,finish=1550586511483763778,duration=7642879
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14d0ff9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2834e0dd
travis_time:start:2834e0dd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:084fe4bd
$ dmesg | grep -i kill
