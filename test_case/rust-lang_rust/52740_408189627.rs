plain
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:51] 
[00:53:51] running 271 tests
[00:53:53] ...........................i.......................F................................................
[00:53:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:53:55] ...............i.......................................................
[00:53:55] failures:
[00:53:55] 
[00:53:55] 
[00:53:55] ---- [parse-fail] parse-fail/extern-foreign-crate.rs stdout ----
[00:53:55] 
[00:53:55] error: /checkout/src/test/parse-fail/extern-foreign-crate.rs:16: unexpected error: '16:18: 16:19: expected one of `-`, `;`, or `as`, found `{`'
[00:53:55] 
[00:53:55] error: /checkout/src/test/parse-fail/extern-foreign-crate.rs:16: expected error not found: expected one of `;` or `as`, found `{`
[00:53:55] error: 1 unexpected errors found, 1 expected errors not found
[00:53:55] status: exit code: 1
[00:53:55] status: exit code: 1
[00:53:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/extern-foreign-crate.--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/parse-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "parse-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:55] 
[00:53:55] 
[00:53:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:55] Build completed unsuccessfully in 0:09:19
[00:53:55] Build completed unsuccessfully in 0:09:19
[00:53:55] make: *** [check] Error 1
[00:53:55] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:134bc2a4
$ date && (curl -fs --head https://Thu, 26 Jul 2018 18:21:57 GMT
---
travis_time:end:18f047be:start=1532629319235534742,finish=1532629319241541656,duration=6006914
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07b18864
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06fbe988
travis_time:start:06fbe988
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b522a78
$ dmesg | grep -i kill
