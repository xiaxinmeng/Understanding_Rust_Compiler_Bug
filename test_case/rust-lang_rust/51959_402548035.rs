plain
[00:57:55] error: /checkout/src/test/compile-fail/issue-20831-debruijn.rs:1: unexpected error: '1:1: 1:1: mismatched types [E0308]'
[00:57:55] 
[00:57:55] error: 2 unexpected errors found, 0 expected errors not found
[00:57:55] status: exit code: 101
[00:57:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-20831-debruijn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20831-debruijn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-20831-debruijn/auxiliary" "-A" "unused"
[00:57:55]     Error {
[00:57:55]         line_num: 1,
[00:57:55]         kind: Some(
[00:57:55]             Error
---
[00:57:55] 
[00:57:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:57:55] 
[00:57:55] 
[00:57:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:55] 
[00:57:55] 
[00:57:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:55] Build completed unsuccessfully in 0:14:36
[00:57:55] Build completed unsuccessfully in 0:14:36
[00:57:55] make: *** [check] Error 1
[00:57:55] Makefile:58: recipe for target 'check' failed
2410096 ./obj
2410064 ./obj/build
1815876 ./obj/build/x86_64-unknown-linux-gnu
730160 ./src
---
144248 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
144244 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
143864 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
129816 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9
129812 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9/s-f2ljfyfeao-lexvjc-2cm8xlrxnbnm8
126680 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
107668 ./obj/build/x86_64-unkno6510885
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_fold:start:after_failure.4
travis_time:start:034330f0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ff943d0
$ dmesg | grep -i kill
