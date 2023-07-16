plain
travis_time:end:03309aaa:start=1548972534773241658,finish=1548972645629062688,duration=110855821030
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
[01:12:50] 
[01:12:50] running 119 tests
[01:13:15] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:19] i......iii.i.....ii
[01:13:19] 
[01:13:19]  finished in 28.453
[01:13:19] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:01] 
[01:17:01] running 292 tests
[01:18:05] ...........................i........................................F............................... 100/292
[01:19:52] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:19:52] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:19:52] ........F...................................................................................
[01:19:52] 
[01:19:52] ---- [rustdoc] rustdoc/index-page.rs stdout ----
[01:19:52] 
[01:19:52] 
[01:19:52] error: htmldocck failed!
[01:19:52] status: exit code: 1
[01:19:52] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/index-page" "/checkout/src/test/rustdoc/index-page.rs"
[01:19:52] ------------------------------------------
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] stderr:
[01:19:52] stderr:
[01:19:52] ------------------------------------------
[01:19:52] 7: @has check failed
[01:19:52]  `XPATH PATTERN` did not match
[01:19:52]  // @has - '//ul[@class="mod"]//a[@href="foo/index.html"]' 'foo'
[01:19:52] Encountered 1 errors
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] 
[01:19:52] 
[01:19:52] thread '[rustdoc] rustdoc/index-page.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:52] 
[01:19:52] ---- [rustdoc] rustdoc/keyword.rs stdout ----
[01:19:52] 
[01:19:52] error: htmldocck failed!
[01:19:52] status: exit code: 1
[01:19:52] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/keyword" "/checkout/src/test/rustdoc/keyword.rs"
[01:19:52] ------------------------------------------
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] stderr:
[01:19:52] stderr:
[01:19:52] ------------------------------------------
[01:19:52] 10: @!has check failed
[01:19:52]  `XPATH PATTERN` did not match
[01:19:52]  // @!has foo/index.html '//a/@href' 'foo/index.html'
[01:19:52] Encountered 1 errors
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] 
---
[01:19:52] test result: FAILED. 288 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out
[01:19:52] 
[01:19:52] 
[01:19:52] 
[01:19:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:52] 
[01:19:52] 
[01:19:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:52] Build completed unsuccessfully in 0:18:28
[01:19:52] Build completed unsuccessfully in 0:18:28
[01:19:52] make: *** [check] Error 1
[01:19:52] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0508ffe0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 31 23:30:48 UTC 2019
---
travis_time:end:05bf9cc8:start=1548977450058911250,finish=1548977450063657664,duration=4746414
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:079b497d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis
