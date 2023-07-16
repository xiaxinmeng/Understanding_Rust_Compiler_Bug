plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:10] 
[01:08:10] running 239 tests
[01:09:34] ....................i...........F...................................................................
[01:10:35] .......................................
[01:10:35] failures:
[01:10:35] 
[01:10:35] 
[01:10:35] ---- [rustdoc] rustdoc/extern-html-root-url.rs stdout ----
[01:10:35] error: rustdoc failed!
[01:10:35] status: exit code: 1
[01:10:35] status: exit code: 1
[01:10:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-html-root-url/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-html-root-url" "/checkout/src/test/rustdoc/extern-html-root-url.rs" "-Z" "unstable-options" "--extern-html-root-url" "https://example.com/core/0.1.0"
[01:10:35] ------------------------------------------
[01:10:35] 
[01:10:35] ------------------------------------------
[01:10:35] stderr:
[01:10:35] stderr:
[01:10:35] ------------------------------------------
[01:10:35] error: --extern-html-root-url must be of the form name=url
[01:10:35] 
[01:10:35] ------------------------------------------
[01:10:35] 
[01:10:35] 
[01:10:35] thread '[rustdoc] rustdoc/extern-html-root-url.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:10:35] 
[01:10:35] 
[01:10:35] failures:
[01:10:35] failures:
[01:10:35]     [rustdoc] rustdoc/extern-html-root-url.rs
[01:10:35] test result: FAILED. 236 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:10:35] 
[01:10:35] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:10:35] 
[01:10:35] 
[01:10:35] 
[01:10:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:35] 
[01:10:35] 
[01:10:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:35] Build completed unsuccessfully in 0:25:36
[01:10:35] Build completed unsuccessfully in 0:25:36
[01:10:35] Makefile:58: recipe for target 'check' failed
[01:10:35] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12feb459
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:027a6ea6:start=1528839150517306165,finish=1528839150524361772,duration=7055607
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10ec87be
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09205dc4
$ dmesg | grep -i kill
