plain
travis_time:end:2a79020a:start=1549671204548580455,finish=1549671304498115103,duration=99949534648
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:38] 
[01:09:38] running 119 tests
[01:10:02] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:06] i......iii.i.....ii
[01:10:06] 
[01:10:06]  finished in 28.888
[01:10:06] travis_fold:end:test_debuginfo

---
[01:16:45] failures:
[01:16:45] 
[01:16:45] ---- [rustdoc] rustdoc/without-redirect.rs stdout ----
[01:16:45] 
[01:16:45] error: htmldocck failed!
[01:16:45] status: exit code: 1
[01:16:45] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/without-redirect" "/checkout/src/test/rustdoc/without-redirect.rs"
[01:16:45] ------------------------------------------
[01:16:45] 
[01:16:45] ------------------------------------------
[01:16:45] stderr:
[01:16:45] stderr:
[01:16:45] ------------------------------------------
[01:16:45] 4: @!has check failed
[01:16:45]  // @!has foo/macro.bar!.html
[01:16:45] Encountered 1 errors
[01:16:45] 
[01:16:45] ------------------------------------------
[01:16:45] 
---
[01:16:45] 
[01:16:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:16:45] 
[01:16:45] 
[01:16:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:45] 
[01:16:45] 
[01:16:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:45] Build completed unsuccessfully in 0:18:39
[01:16:45] Build completed unsuccessfully in 0:18:39
[01:16:45] make: *** [check] Error 1
[01:16:45] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34fbdff8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 01:31:58 UTC 2019
---
travis_time:end:002aa29c:start=1549675920537299506,finish=1549675920542180779,duration=4881273
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014df626
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02035740
travis_time:start:02035740
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2055300c
$ dmesg | grep -i kill
