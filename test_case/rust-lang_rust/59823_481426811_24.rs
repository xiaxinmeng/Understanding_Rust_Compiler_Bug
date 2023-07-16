\n\nThis will fail because the compiler does not know which instance of `Foo` to\ncall `bar` on. Change `Foo::bar()` to `Foo::<T>::bar()` to resolve the error.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-59001.rs","byte_start":153,"byte_end":165,"line_start":9,"line_end":9,"column_start":26,"column_end":38,"is_primary":true,"text":[{"text":"async fn enter<'a, F, R>(mut callback: F)","highlight_start":26,"highlight_end":38}],"label":"cannot infer type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0282]: type annotations needed\n  --> /checkout/src/test/ui/issues/issue-59001.rs:9:26\n   |\nLL | async fn enter<'a, F, R>(mut callback: F)\n   |                          ^^^^^^^^^^^^ cannot infer type\n\n"}
[01:07:25] {"message":"For more information about this error, try `rustc --explain E0282`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0282`.\n"}
[01:07:25] 
[01:07:25] ------------------------------------------
[01:07:25] 
---
[01:07:25] 
[01:07:25] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:07:25] 
[01:07:25] 
[01:07:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:25] 
[01:07:25] 
[01:07:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:25] Build completed unsuccessfully in 0:04:25
[01:07:25] Build completed unsuccessfully in 0:04:25
[01:07:25] make: *** [check] Error 1
[01:07:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09558c42
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  9 20:40:14 UTC 2019
---
travis_time:end:0fc3eff1:start=1554842416043625610,finish=1554842416050980098,duration=7354488
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:35a85878
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03016640
$ dmesg | grep -i kill
