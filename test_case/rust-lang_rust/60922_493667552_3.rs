\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":215,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo((), drop)","highlight_start":5,"highlight_end":8}],"label":"expected bound lifetime parameter 'a, found concrete lifetime","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":91,"byte_end":197,"line_start":9,"line_end":11,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"pub fn foo<T, F>(_: T, _: F)","highlight_start":1,"highlight_end":29},{"text":"where T: for<'a> Trait<'a>,","highlight_start":1,"highlight_end":28},{"text":"      F: for<'a> FnMut(<T as Trait<'a>>::Item) {}","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0271]: type mismatch resolving `for<'a> <fn(_) {std::mem::drop::<_>} as std::ops::FnOnce<(<() as Trait<'a>>::Item,)>>::Output == ()`\n  --> /checkout/src/test/ui/issues/issue-60283.rs:14:5\n   |\nLL |     foo((), drop)\n   |     ^^^ expected bound lifetime parameter 'a, found concrete lifetime\n   |\nnote: required by `foo`\n  --> /checkout/src/test/ui/issues/issue-60283.rs:9:1\n   |\nLL | / pub fn foo<T, F>(_: T, _: F)\nLL | | where T: for<'a> Trait<'a>,\nLL | |       F: for<'a> FnMut(<T as Trait<'a>>::Item) {}\n   | |_________________________________________________^\n\n"}
[01:11:13] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:11:13] {"message":"Some errors occurred: E0271, E0631.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0271, E0631.\n"}
[01:11:13] 
[01:11:13] ------------------------------------------
[01:11:13] 
[01:11:13] thread '[ui] ui/issues/issue-60283.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:11:13] 
[01:11:13] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:11:13] 
[01:11:13] 
[01:11:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:13] 
[01:11:13] 
[01:11:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:13] Build completed unsuccessfully in 0:04:36
[01:11:13] Build completed unsuccessfully in 0:04:36
[01:11:13] make: *** [check] Error 1
[01:11:13] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25b27faa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 18 10:47:01 UTC 2019
---
travis_time:end:053f5794:start=1558176422535579302,finish=1558176422543479201,duration=7899899
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a457316
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b356a76
$ dmesg | grep -i kill
