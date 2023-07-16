plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:12] 
[00:55:12] running 246 tests
[00:55:39] .........F...........i..............................................................................
[00:56:05] ..............................................
[00:56:05] failures:
[00:56:05] 
[00:56:05] ---- [rustdoc] rustdoc/const-evalutation-ice.rs stdout ----
[00:56:05] ---- [rustdoc] rustdoc/const-evalutation-ice.rs stdout ----
[00:56:05] 
[00:56:05] error: rustdoc failed!
[00:56:05] status: exit code: 1
[00:56:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-evalutation-ice/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-evalutation-ice" "/checkout/src/test/rustdoc/const-evalutation-ice.rs"
[00:56:05] ------------------------------------------
[00:56:05] 
[00:56:05] ------------------------------------------
[00:56:05] stderr:
---
[00:56:05] 
[00:56:05] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:05] 
[00:56:05] 
[00:56:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/nativ% /
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
none            768M     0  768M   0% /var/ramfs
---
travis_time:end:0cb37200:start=1532466975755991964,finish=1532466975765231114,duration=9239150
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0035447a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09615120
travis_time:start:09615120
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ed8ee60
$ dmesg | grep -i kill
