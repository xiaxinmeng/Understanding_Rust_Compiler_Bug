plain
travis_time:end:0cf62d48:start=1559051331972176186,finish=1559051421095330445,duration=89123154259
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:57] 
[01:23:57] running 143 tests
[01:23:59] i..iii.....iii..iiii.....i......................i...i................i.....i..........ii.i..i..i.ii. 100/143
[01:24:01] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:24:01] 
[01:24:01]  finished in 4.910
[01:24:01] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:04] 
[01:24:04] running 9 tests
[01:24:04] iiiiiiiii
[01:24:04] 
[01:24:04]  finished in 0.169
[01:24:04] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:20] 
[01:24:20] running 122 tests
[01:24:47] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:24:52] .i.i......iii.i.....ii
[01:24:52] 
[01:24:52]  finished in 31.600
[01:24:52] travis_fold:end:test_debuginfo

---
[01:29:04] 
[01:29:04] running 309 tests
[01:30:24] .............................i...................................................................... 100/309
[01:31:32] ..............................................i..................................................... 200/309
[01:32:37] .........................................................................................F.......... 300/309
[01:32:42] failures:
[01:32:42] 
[01:32:42] ---- [rustdoc] rustdoc/trait-attributes.rs stdout ----
[01:32:42] 
[01:32:42] 
[01:32:42] error: htmldocck failed!
[01:32:42] status: exit code: 1
[01:32:42] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-attributes" "/checkout/src/test/rustdoc/trait-attributes.rs"
[01:32:42] stdout:
[01:32:42] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:32:42] ------------------------------------------
[01:32:42] 
[01:32:42] 
[01:32:42] ------------------------------------------
[01:32:42] stderr:
[01:32:42] ------------------------------------------
[01:32:42] 6: @has check failed
[01:32:42]  `XPATH PATTERN` did not match
[01:32:42]      // @has foo/trait.Foo.html '//h3[@id="tymethod.foo"]//div[@class="docblock attributes"]' '#[must_use]'
[01:32:42] 15: @has check failed
[01:32:42]  `XPATH PATTERN` did not match
[01:32:42]      // @has foo/struct.Bar.html '//h4[@id="method.bar"]//div[@class="docblock attributes"]' '#[must_use]'
[01:32:42] 19: @has check failed
[01:32:42]  `XPATH PATTERN` did not match
[01:32:42]      // @has foo/struct.Bar.html '//h4[@id="method.bar2"]//div[@class="docblock attributes"]' '#[must_use]'
[01:32:42] Encountered 3 errors
[01:32:42] 
[01:32:42] ------------------------------------------
[01:32:42] 
---
[01:32:42] test result: FAILED. 306 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:32:42] 
[01:32:42] 
[01:32:42] 
[01:32:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:32:42] 
[01:32:42] 
[01:32:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:42] Build completed unsuccessfully in 0:21:09
[01:32:42] Build completed unsuccessfully in 0:21:09
[01:32:42] make: *** [check] Error 1
[01:32:42] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b36fb8e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 28 15:23:13 UTC 2019
---
travis_time:end:09145d53:start=1559056994768923504,finish=1559056994774354286,duration=5430782
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29aac1c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20535df2
travis_time:start:20535df2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13c66180
$ dmesg | grep -i kill
