\n\nDelete the offending feature attribute, or add it to the list of allowed\nfeatures in the `-Z allow_features` flag.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gate/allow-features.rs","byte_start":190,"byte_end":210,"line_start":6,"line_end":6,"column_start":12,"column_end":32,"is_primary":true,"text":[{"text":"#![feature(rustc_const_unstable)] //~ ERROR not in the list of allowed features","highlight_start":12,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0725]: the feature `rustc_const_unstable` is not in the list of allowed features\n  --> /checkout/src/test/ui/feature-gate/allow-features.rs:6:12\n   |\nLL | #![feature(rustc_const_unstable)] //~ ERROR not in the list of allowed features\n   |            ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:17:00] {"message":"For more information about this error, try `rustc --explain E0725`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0725`.\n"}
[01:17:00] 
[01:17:00] ------------------------------------------
[01:17:00] 
---
[01:17:00] 
[01:17:00] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:17:00] 
[01:17:00] 
[01:17:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:00] 
[01:17:00] 
[01:17:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:00] Build completed unsuccessfully in 0:04:47
[01:17:00] Build completed unsuccessfully in 0:04:47
[01:17:00] make: *** [check] Error 1
[01:17:00] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3152fe6f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 14 02:08:05 UTC 2019
---
travis_time:end:0a0a6c49:start=1552529287521231908,finish=1552529287529670148,duration=8438240
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:121e3670
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cc034b2
$ dmesg | grep -i kill
