plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:00] 
[01:06:00] running 240 tests
[01:07:15] ....................i..................................F.........................................FFF
[01:08:00] .F...........i......................................................................................
nu/test/rustdoc/issue-20727" "/checkout/src/test/rustdoc/issue-20727.rs"
[01:08:11] ------------------------------------------
[01:08:11] 
[01:08:11] ------------------------------------------
[01:08:11] stderr:
[01:08:11] stderr:
[01:08:11] ------------------------------------------
[01:08:11] 31: @has check failed
[01:08:11]  `XPATH PATTERN` did not match
[01:08:11]      // @has - '//*[@class="rust trait"]' "fn deref(&'a self) -> &'a Self::Target;"
[01:08:11] Encountered 1 errors
[01:08:11] 
[01:08:11] ------------------------------------------
[01:08:11] 
---
[01:08:11] 
[01:08:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:08:11] 
[01:08:11] 
[01:08:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:11] 
[01:08:11] 
[01:08:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:11] Build completed unsuccessfully in 0:23:51
[01:08:11] Build completed unsuccessfully in 0:23:51
[01:08:11] make: *** [check] Error 1
[01:08:11] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0236e4b6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jun 23 21:30:33 UTC 2018
Sat, 23 Jun 2018 21:30:33 GMT
travis_time:end:0236e4b6:start=1529789433636410920,fin89435643665249,duration=7632151
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cfb6ec9
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bebf10b
$ dmesg | grep -i kill
