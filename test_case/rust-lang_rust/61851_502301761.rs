plain
travis_time:end:2c785ddc:start=1560547408457481260,finish=1560547409331830738,duration=874349478
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
[01:06:06] 
[01:06:06] running 9 tests
[01:06:06] iiiiiiiii
[01:06:06] 
[01:06:06]  finished in 0.152
[01:06:06] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:22] 
[01:06:22] running 122 tests
[01:06:48] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:53] .i.i......iii.i.....ii
[01:06:53] 
[01:06:53]  finished in 30.491
[01:06:53] travis_fold:end:test_debuginfo

---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:58] 
[01:12:58] running 312 tests
[01:14:07] ................................i................................................................... 100/312
[01:15:08] ..................F.............................i................................................... 200/312
[01:16:15] ............
[01:16:15] failures:
[01:16:15] 
[01:16:15] ---- [rustdoc] rustdoc/issue-15318.rs stdout ----
[01:16:15] ---- [rustdoc] rustdoc/issue-15318.rs stdout ----
[01:16:15] 
[01:16:15] error: htmldocck failed!
[01:16:15] status: exit code: 1
[01:16:15] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15318" "/checkout/src/test/rustdoc/issue-15318.rs"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] 8: @has check failed
[01:16:15]  `XPATH PATTERN` did not match
[01:16:15]  // @has issue_15318/fn.bar.html '//*[@href="http://example.com/issue_15318/primitive.pointer.html"]' '*mut T'
[01:16:15] Encountered 1 errors
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
---
[01:16:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:16:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:15] 
[01:16:15] 
[01:16:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:15] 
[01:16:15] 
[01:16:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:15] Build completed unsuccessfully in 1:11:28
---
travis_time:end:2d1fdbc8:start=1560551997851760869,finish=1560551997857412595,duration=5651726
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c666200
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07266022
travis_time:start:07266022
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b896df3
$ dmesg | grep -i kill
