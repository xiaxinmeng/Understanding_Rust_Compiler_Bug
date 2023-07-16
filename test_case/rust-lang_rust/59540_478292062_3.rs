\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-21177.rs","byte_start":61,"byte_end":65,"line_start":6,"line_end":6,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"fn foo<T: Trait<A = T::B>>() { }","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires computing the bounds for type parameter `T`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-21177.rs","byte_start":61,"byte_end":65,"line_start":6,"line_end":6,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"fn foo<T: Trait<A = T::B>>() { }","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the bounds for type parameter `T`\n  --> /checkout/src/test/ui/issues/issue-21177.rs:6:21\n   |\nLL | fn foo<T: Trait<A = T::B>>() { }\n   |                     ^^^^\n   |\n   = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle\nnote: cycle used when processing `foo`\n  --> /checkout/src/test/ui/issues/issue-21177.rs:6:21\n   |\nLL | fn foo<T: Trait<A = T::B>>() { }\n   |                     ^^^^\n\n"}
[01:10:42] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:10:42] 
[01:10:42] ------------------------------------------
[01:10:42] 
---
[01:10:42] 
[01:10:42] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:10:42] 
[01:10:42] 
[01:10:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:42] 
[01:10:42] 
[01:10:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:42] Build completed unsuccessfully in 0:05:00
[01:10:42] Build completed unsuccessfully in 0:05:00
[01:10:42] make: *** [check] Error 1
[01:10:42] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:29a9a308
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 30 21:48:04 UTC 2019
---
travis_time:end:24d6d63a:start=1553982486506860227,finish=1553982486515825617,duration=8965390
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04992e20
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22569c90
$ dmesg | grep -i kill
