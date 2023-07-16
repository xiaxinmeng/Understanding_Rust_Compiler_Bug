plain
travis_time:end:0140d278:start=1555621702569470955,finish=1555621825043885335,duration=122474414380
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
[01:18:14] 
[01:18:14] running 9 tests
[01:18:14] iiiiiiiii
[01:18:14] 
[01:18:14]  finished in 0.158
[01:18:14] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:31] 
[01:18:31] running 121 tests
[01:18:58] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:19:02] i.i......iii.i.....ii
[01:19:02] 
[01:19:02]  finished in 31.465
[01:19:02] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:01] 
[01:23:01] running 304 tests
[01:24:13] .....F......................i....................................................................... 100/304
[01:26:22] .................................................................................................... 300/304
[01:26:24] ....
[01:26:24] failures:
[01:26:24] 
[01:26:24] 
[01:26:24] ---- [rustdoc] rustdoc/attributes.rs stdout ----
[01:26:24] 
[01:26:24] error: htmldocck failed!
[01:26:24] status: exit code: 1
[01:26:24] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/attributes" "/checkout/src/test/rustdoc/attributes.rs"
[01:26:24] ------------------------------------------
[01:26:24] 
[01:26:24] ------------------------------------------
[01:26:24] stderr:
[01:26:24] stderr:
[01:26:24] ------------------------------------------
[01:26:24] 11: @has check failed
[01:26:24]  `XPATH PATTERN` did not match
[01:26:24]  // @has foo/enum.Foo.html '//*[@class="docblock attributes"]' '#[repr(i64)]'
[01:26:24] 12: @has check failed
[01:26:24]  `XPATH PATTERN` did not match
[01:26:24]  // @has foo/enum.Foo.html '//*[@class="docblock attributes"]' '#[must_use]'
[01:26:24] Encountered 2 errors
[01:26:24] 
[01:26:24] ------------------------------------------
[01:26:24] 
---
[01:26:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:26:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:26:24] 
[01:26:24] 
[01:26:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:24] 
[01:26:24] 
[01:26:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:24] Build completed unsuccessfully in 0:20:34
[01:26:24] Build completed unsuccessfully in 0:20:34
[01:26:24] make: *** [check] Error 1
[01:26:24] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:081c847a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 22:36:59 UTC 2019
---
travis_time:end:0df58f80:start=1555627021113141417,finish=1555627021169388909,duration=56247492
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0459ce56
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0682ab4c
$ dmesg | grep -i kill
