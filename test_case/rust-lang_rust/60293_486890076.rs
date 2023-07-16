plain
travis_time:end:04bf2bd3:start=1556235558705585229,finish=1556235559498044560,duration=792459331
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:49] 
[01:23:49] running 9 tests
[01:23:49] iiiiiiiii
[01:23:49] 
[01:23:49]  finished in 0.150
[01:23:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:05] 
[01:24:05] running 121 tests
[01:24:30] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:24:34] i.i......iii.i.....ii
[01:24:34] 
[01:24:34]  finished in 29.331
[01:24:34] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:28:34] 
[01:28:34] running 304 tests
[01:29:49] ............................i...............F....................................................... 100/304
[01:31:59] .................................................................................................... 300/304
[01:32:01] ....
[01:32:01] failures:
[01:32:01] 
[01:32:01] 
[01:32:01] ---- [rustdoc] rustdoc/empty-section.rs stdout ----
[01:32:01] 
[01:32:01] error: htmldocck failed!
[01:32:01] status: exit code: 1
[01:32:01] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-section" "/checkout/src/test/rustdoc/empty-section.rs"
[01:32:01] ------------------------------------------
[01:32:01] 
[01:32:01] ------------------------------------------
[01:32:01] stderr:
[01:32:01] stderr:
[01:32:01] ------------------------------------------
[01:32:01] 8: @!has check failed
[01:32:01]  `PATTERN` did not match
[01:32:01]  // @!has - 'Auto Trait Implementations'
[01:32:01] Encountered 1 errors
[01:32:01] 
[01:32:01] ------------------------------------------
[01:32:01] 
---
[01:32:01] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:32:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:32:01] 
[01:32:01] 
[01:32:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:32:01] 
[01:32:01] 
[01:32:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:01] Build completed unsuccessfully in 0:20:08
[01:32:01] Build completed unsuccessfully in 0:20:08
[01:32:01] make: *** [check] Error 1
[01:32:01] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03b1611e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 26 01:11:32 UTC 2019
---
travis_time:end:00209284:start=1556241094209385924,finish=1556241094214291889,duration=4905965
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2adb8434
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02104671
travis_time:start:02104671
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:211225e0
$ dmesg | grep -i kill
