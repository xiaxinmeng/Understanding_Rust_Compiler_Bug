\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference:[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[00:52:38] ------------------------------------------
[00:52:38] 
[00:52:38] thread '[ui] ui/nll/issue-53807.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:52:38] 
---
[00:52:38] test result: FAILED. 4966 passed; 6 failed; 24 ignored; 0 measured; 0 filtered out
[00:52:38] 
[00:52:38] 
[00:52:38] 
[00:52:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:38] 
[00:52:38] 
[00:52:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:38] Build completed unsuccessfully in 0:03:49
[00:52:38] Build completed unsuccessfully in 0:03:49
[00:52:38] Makefile:58: recipe for target 'check' failed
[00:52:38] make: *** [check] Error 1
2344532 ./obj
2344492 ./obj/build
1708724 ./obj/build/x86_64-unknown-linux-gnu
1193616 ./.git
---
149112 ./src/llvm-emscripten/test
142856 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
137072 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
137068 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
134668 ./obj/build/bootstrap/debug/incremental/ravis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11fa9e42
travis_time:start:11fa9e42
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01aa3540
$ dmesg | grep -i kill
