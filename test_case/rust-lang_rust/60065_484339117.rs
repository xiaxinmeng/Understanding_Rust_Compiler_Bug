plain
travis_time:end:003ac09e:start=1555551905730415659,finish=1555551909614020716,duration=3883605057
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
[01:15:23] 
[01:15:23] running 9 tests
[01:15:23] iiiiiiiii
[01:15:23] 
[01:15:23]  finished in 0.151
[01:15:23] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:39] 
[01:15:39] running 121 tests
[01:16:05] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:16:09] i.i......iii.i.....ii
[01:16:09] 
[01:16:09]  finished in 30.341
[01:16:09] travis_fold:end:test_debuginfo

---
[01:20:02] 
[01:20:02] running 304 tests
[01:21:14] ............................i....................................................................... 100/304
[01:22:15] ...........................................i........................................................ 200/304
[01:23:19] ..............................F..F.................................................................. 300/304
[01:23:21] failures:
[01:23:21] 
[01:23:21] ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
[01:23:21] 
[01:23:21] 
[01:23:21] error: htmldocck failed!
[01:23:21] status: exit code: 1
[01:23:21] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs"
[01:23:21] ------------------------------------------
[01:23:21] 
[01:23:21] ------------------------------------------
[01:23:21] stderr:
[01:23:21] stderr:
[01:23:21] ------------------------------------------
[01:23:21] 14: @matches check failed
[01:23:21]  `XPATH PATTERN` did not match
[01:23:21]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=%23!%5Ballow(unused)%5D%0Aextern%20crate%20foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D"]' "Run"
[01:23:21] Encountered 1 errors
[01:23:21] 
[01:23:21] ------------------------------------------
[01:23:21] 
[01:23:21] 
[01:23:21] 
[01:23:21] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:23:21] 
[01:23:21] error: htmldocck failed!
[01:23:21] status: exit code: 1
[01:23:21] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground" "/checkout/src/test/rustdoc/playground.rs"
[01:23:21] ------------------------------------------
[01:23:21] 
[01:23:21] ------------------------------------------
[01:23:21] stderr:
[01:23:21] stderr:
[01:23:21] ------------------------------------------
[01:23:21] 27: @matches check failed
[01:23:21]  `XPATH PATTERN` did not match
[01:23:21]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0A%20%20%20%20println!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:23:21] 28: @matches check failed
[01:23:21]  `XPATH PATTERN` did not match
[01:23:21]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:23:21] 29: @matches check failed
[01:23:21]  `XPATH PATTERN` did not match
[01:23:21]  // @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Ballow(unused)%5D%0A%23!%5Bfeature(something)%5D%0A%0Afn%20main()%20%7B%0A%20%20%20%20println!(%22Hello%2C%20world!%22)%3B%0A%7D&version=nightly"]' "Run"
[01:23:21] Encountered 3 errors
[01:23:21] 
[01:23:21] ------------------------------------------
[01:23:21] 
---
[01:23:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:23:21] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:23:21] 
[01:23:21] 
[01:23:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:21] 
[01:23:21] 
[01:23:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:21] Build completed unsuccessfully in 0:19:42
[01:23:21] Build completed unsuccessfully in 0:19:42
[01:23:21] Makefile:48: recipe for target 'check' failed
[01:23:21] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25d47e60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 03:08:42 UTC 2019
---
travis_time:end:2d083b20:start=1555556923883078720,finish=1555556923887827493,duration=4748773
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:004b10f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0331cc6b
travis_time:start:0331cc6b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0459b6ac
$ dmesg | grep -i kill
