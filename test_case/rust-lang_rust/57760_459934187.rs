plain
travis_time:end:02e5c552:start=1549076675187706943,finish=1549076758224170824,duration=83036463881
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
[01:10:52] 
[01:10:52] running 119 tests
[01:11:19] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:24] i......iii.i.....ii
[01:11:24] 
[01:11:24]  finished in 31.912
[01:11:24] travis_fold:end:test_debuginfo

---
[01:15:11] 
[01:15:11] running 292 tests
[01:16:18] ...........................i........................................................................ 100/292
[01:17:15] .......................................i............................................................ 200/292
[01:18:10] .......................................................................................F....
[01:18:10] 
[01:18:10] ---- [rustdoc] rustdoc/variadic.rs stdout ----
[01:18:10] 
[01:18:10] 
[01:18:10] error: htmldocck failed!
[01:18:10] status: exit code: 1
[01:18:10] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/variadic" "/checkout/src/test/rustdoc/variadic.rs"
[01:18:10] ------------------------------------------
[01:18:10] 
[01:18:10] ------------------------------------------
[01:18:10] stderr:
[01:18:10] stderr:
[01:18:10] ------------------------------------------
[01:18:10] 2: @has check failed
[01:18:10]  `XPATH PATTERN` did not match
[01:18:10]      // @has variadic/fn.foo.html //pre 'pub unsafe extern "C" fn foo(x: i32, ...)'
[01:18:10] Encountered 1 errors
[01:18:10] 
[01:18:10] ------------------------------------------
[01:18:10] 
---
[01:18:10] 
[01:18:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:18:10] 
[01:18:10] 
[01:18:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:10] 
[01:18:10] 
[01:18:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:10] Build completed unsuccessfully in 0:19:29
[01:18:10] Build completed unsuccessfully in 0:19:29
[01:18:10] make: *** [check] Error 1
[01:18:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:22390928
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 04:24:18 UTC 2019
---
travis_time:end:29279c4c:start=1549081460087253575,finish=1549081460092232107,duration=4978532
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d0c7982
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00a2d544
travis_time:start:00a2d544
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04586046
$ dmesg | grep -i kill
