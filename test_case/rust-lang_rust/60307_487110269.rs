plain
travis_time:end:35e00c32:start=1556288750291620017,finish=1556288752783072256,duration=2491452239
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
[01:26:12] 
[01:26:12] running 9 tests
[01:26:12] iiiiiiiii
[01:26:12] 
[01:26:12]  finished in 0.152
[01:26:12] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:28] 
[01:26:28] running 121 tests
[01:26:54] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:26:58] i.i......iii.i.....ii
[01:26:58] 
[01:26:58]  finished in 29.807
[01:26:58] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:31:26] 
[01:31:26] running 304 tests
[01:32:46] .........F..................i...............................F...F................................... 100/304
[01:33:54] ...........................................i........F............................................... 200/304
[01:35:06] ....................................F............................................................... 300/304
[01:35:08] failures:
[01:35:08] 
[01:35:08] ---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----
[01:35:08] 
[01:35:08] 
[01:35:08] error: htmldocck failed!
[01:35:08] status: exit code: 1
[01:35:08] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] stderr:
[01:35:08] stderr:
[01:35:08] ------------------------------------------
[01:35:08] 3: @has check failed
[01:35:08]  `XPATH PATTERN` did not match
[01:35:08]  // @has foo/struct.S.html '//h3[@id="impl-Into"]//code' 'impl<T, U> Into for T'
[01:35:08] Encountered 1 errors
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] 
[01:35:08] 
[01:35:08] ---- [rustdoc] rustdoc/generic-impl.rs stdout ----
[01:35:08] 
[01:35:08] error: htmldocck failed!
[01:35:08] status: exit code: 1
[01:35:08] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] stderr:
[01:35:08] stderr:
[01:35:08] ------------------------------------------
[01:35:08] 8: @has check failed
[01:35:08]  `XPATH PATTERN` did not match
[01:35:08]  // @has foo/struct.Foo.html '//h3[@id="impl-ToString"]//code' 'impl<T> ToString for T'
[01:35:08] Encountered 1 errors
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] 
[01:35:08] 
[01:35:08] ---- [rustdoc] rustdoc/hidden-trait-struct-impls.rs stdout ----
[01:35:08] 
[01:35:08] error: htmldocck failed!
[01:35:08] status: exit code: 1
[01:35:08] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-struct-impls" "/checkout/src/test/rustdoc/hidden-trait-struct-impls.rs"
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] stderr:
[01:35:08] stderr:
[01:35:08] ------------------------------------------
[01:35:08] 18: @has check failed
[01:35:08]  `XPATH PATTERN` did not match
[01:35:08]  // @has foo/struct.Bar.html '//*[@id="impl-Bam"]' 'impl Bam for Bar'
[01:35:08] Encountered 1 errors
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] 
[01:35:08] 
[01:35:08] ---- [rustdoc] rustdoc/issue-29503.rs stdout ----
[01:35:08] 
[01:35:08] error: htmldocck failed!
[01:35:08] status: exit code: 1
[01:35:08] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] stderr:
[01:35:08] stderr:
[01:35:08] ------------------------------------------
[01:35:08] 10: @has check failed
[01:35:08]  `XPATH PATTERN` did not match
[01:35:08]  // @has - "//div[@id='implementors-list']/h3[@id='impl-MyTrait']//code" "impl<T> MyTrait for T where T: Debug"
[01:35:08] Encountered 1 errors
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] 
[01:35:08] 
[01:35:08] ---- [rustdoc] rustdoc/primitive-generic-impl.rs stdout ----
[01:35:08] 
[01:35:08] error: htmldocck failed!
[01:35:08] status: exit code: 1
[01:35:08] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive-generic-impl.rs"
[01:35:08] ------------------------------------------
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] stderr:
[01:35:08] stderr:
[01:35:08] ------------------------------------------
[01:35:08] 5: @has check failed
[01:35:08]  `XPATH PATTERN` did not match
[01:35:08]  // @has foo/primitive.i32.html '//h3[@id="impl-ToString"]//code' 'impl<T> ToString for T'
[01:35:08] Encountered 1 errors
[01:35:08] 
[01:35:08] ------------------------------------------
[01:35:08] 
---
[01:35:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:35:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:35:08] 
[01:35:08] 
[01:35:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:35:08] 
[01:35:08] 
[01:35:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:08] Build completed unsuccessfully in 0:21:20
[01:35:08] Build completed unsuccessfully in 0:21:20
[01:35:08] make: *** [check] Error 1
[01:35:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06e6be70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 26 16:01:12 UTC 2019
---
travis_time:end:2e219839:start=1556294474306746498,finish=1556294474311176498,duration=4430000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03436480
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14aa5288
travis_time:start:14aa5288
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:112a1cbb
$ dmesg | grep -i kill
