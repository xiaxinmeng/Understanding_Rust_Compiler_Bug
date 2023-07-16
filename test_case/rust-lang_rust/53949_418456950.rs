plain
[00:48:57] ....................................................................................................
[00:49:00] ....................................................................................................
[00:49:03] ...........................i........................................................................
[00:49:05] ....................................................................................................
[00:49:08] ............................................................................iiiiiiiii...............
[00:49:13] ....................................................................................................
[00:49:17] ....................................................................................................
[00:49:20] .........................................................i..........................................
[00:49:23] ....................................................................................................
---
travis_time:start:test_parse-fail
Check compiletest suite=parse-fail mode=parse-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:34] 
[00:55:34] running 274 tests
[00:55:35] ..............................i.........................................................F...........
[00:55:38] ..................i.......................................................
[00:55:38] failures:
[00:55:38] 
[00:55:38] ---- [parse-fail] parse-fail/issue-2354.rs stdout ----
[00:55:38] ---- [parse-fail] parse-fail/issue-2354.rs stdout ----
[00:55:38] 
[00:55:38] error: /checkout/src/test/parse-fail/issue-2354.rs:13: expected help message not found: did you mean to close this delimiter?
[00:55:38] 
[00:55:38] error: 0 unexpected errors found, 1 expected errors not found
[00:55:38] status: exit code: 1
[00:55:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/issue-2354.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/issue-2354/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_6inux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:38] 
[00:55:38] 
[00:55:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:38] Build completed unsuccessfully in 0:10:39
[00:55:38] Build completed unsuccessfully in 0:10:39
[00:55:38] make: *** [check] Error 1
[00:55:38] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:064f7fee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:000aa2ca:start=1536082994321668769,finish=1536082994397918102,duration=76249333
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:182a432b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f830e0a
$ dmesg | grep -i kill
