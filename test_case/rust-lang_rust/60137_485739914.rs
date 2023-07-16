plain
travis_time:end:0e2d1d9f:start=1556008525833818943,finish=1556008662798800767,duration=136964981824
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
[01:17:55] 
[01:17:55] running 9 tests
[01:17:55] iiiiiiiii
[01:17:55] 
[01:17:55]  finished in 0.147
[01:17:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:11] 
[01:18:11] running 121 tests
[01:18:35] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:39] i.i......iii.i.....ii
[01:18:39] 
[01:18:39]  finished in 28.447
[01:18:39] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:54] 
[01:22:54] running 304 tests
[01:24:15] ..........F.................i....................................................................... 100/304
[01:25:23] ...........................................i.........................F.............................. 200/304
[01:26:36] ......................................................................F............................. 300/304
[01:26:38] failures:
[01:26:38] 
[01:26:38] ---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----
[01:26:38] 
[01:26:38] 
[01:26:38] error: htmldocck failed!
[01:26:38] status: exit code: 1
[01:26:38] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
[01:26:38] ------------------------------------------
[01:26:38] 
[01:26:38] ------------------------------------------
[01:26:38] stderr:
[01:26:38] stderr:
[01:26:38] ------------------------------------------
[01:26:38] 3: @has check failed
[01:26:38]  `XPATH PATTERN` did not match
[01:26:38]  // @has foo/struct.S.html '//h3[@id="impl-Into"]//code' 'impl<T, U> Into for T'
[01:26:38] Encountered 1 errors
[01:26:38] 
[01:26:38] ------------------------------------------
[01:26:38] 
[01:26:38] 
[01:26:38] 
[01:26:38] ---- [rustdoc] rustdoc/issue-34473.rs stdout ----
[01:26:38] 
[01:26:38] error: htmldocck failed!
[01:26:38] status: exit code: 1
[01:26:38] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34473" "/checkout/src/test/rustdoc/issue-34473.rs"
[01:26:38] ------------------------------------------
[01:26:38] 
[01:26:38] ------------------------------------------
[01:26:38] stderr:
[01:26:38] stderr:
[01:26:38] ------------------------------------------
[01:26:38] 10: @!has check failed
[01:26:38]  `PATTERN` did not match
[01:26:38]  // @!has - SomeTypeWithLongName
[01:26:38] Encountered 1 errors
[01:26:38] 
[01:26:38] ------------------------------------------
[01:26:38] 
[01:26:38] 
[01:26:38] 
[01:26:38] ---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----
[01:26:38] 
[01:26:38] error: htmldocck failed!
[01:26:38] status: exit code: 1
[01:26:38] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex" "/checkout/src/test/rustdoc/synthetic_auto/complex.rs"
[01:26:38] ------------------------------------------
[01:26:38] 
[01:26:38] ------------------------------------------
[01:26:38] stderr:
[01:26:38] stderr:
[01:26:38] ------------------------------------------
[01:26:38] 23: @has check failed
[01:26:38]  `XPATH PATTERN` did not match
[01:26:38]  // @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]//code' "impl<'a, T, K: ?Sized> Send for NotOuter<'a, T, K> where K: for<'b> Fn((&'b bool, &'a u8)) -> &'b i8, T: MyTrait<'a>, <T as MyTrait<'a>>::MyItem: Copy, 'a: 'static"
[01:26:38] Encountered 1 errors
[01:26:38] 
[01:26:38] ------------------------------------------
[01:26:38] 
---
[01:26:38] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:26:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:26:38] 
[01:26:38] 
[01:26:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:38] 
[01:26:38] 
[01:26:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:38] Build completed unsuccessfully in 0:20:44
[01:26:38] Build completed unsuccessfully in 0:20:44
[01:26:38] make: *** [check] Error 1
[01:26:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00260b01
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 10:04:30 UTC 2019
