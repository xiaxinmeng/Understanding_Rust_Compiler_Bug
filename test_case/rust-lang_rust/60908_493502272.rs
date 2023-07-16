plain
travis_time:end:02abc76a:start=1558101271490210871,finish=1558101371101567677,duration=99611356806
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
[01:25:06] 
[01:25:06] running 143 tests
[01:25:09] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:25:11] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:25:11] 
[01:25:11]  finished in 4.858
[01:25:11] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:13] 
[01:25:13] running 9 tests
[01:25:13] iiiiiiiii
[01:25:13] 
[01:25:13]  finished in 0.167
[01:25:13] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:30] 
[01:25:30] running 122 tests
[01:25:56] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:26:01] .i.i......iii.i.....ii
[01:26:01] 
[01:26:01]  finished in 31.376
[01:26:01] travis_fold:end:test_debuginfo

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:50:35] 
[01:50:35] running 27 tests
[01:50:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:50:50] .............F..........F..
[01:50:50] 
[01:50:50] ---- [ui] rustdoc-ui/doc-without-codeblock.rs stdout ----
[01:50:50] diff of stderr:
[01:50:50] 
[01:50:50] 
[01:50:50] 4 LL | / #![deny(missing_doc_code_examples)]
[01:50:50] 6 LL | | /// Some docs.
[01:50:50] 6 LL | | /// Some docs.
[01:50:50] - LL | | //~^ ERROR Missing code example in this documentation
[01:50:50] + LL | |
[01:50:50] 8 ...  |
[01:50:50] 9 LL | |     pub fn bar() {}
[01:50:50] 
[01:50:50] 
[01:50:50] The actual stderr differed from the expected stderr.
[01:50:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/doc-without-codeblock.stderr
[01:50:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/doc-without-codeblock.stderr
[01:50:50] To update references, rerun the tests and pass the `--bless` flag
[01:50:50] To only update this specific test, also pass `--test-args doc-without-codeblock.rs`
[01:50:50] error: 1 errors occurred comparing output.
[01:50:50] status: exit code: 1
[01:50:50] status: exit code: 1
[01:50:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-without-codeblock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/auxiliary"
[01:50:50] ------------------------------------------
[01:50:50] 
[01:50:50] ------------------------------------------
[01:50:50] stderr:
[01:50:50] stderr:
[01:50:50] ------------------------------------------
[01:50:50] error: Missing code example in this documentation
[01:50:50]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:3:1
[01:50:50]    |
[01:50:50] LL | / #![deny(missing_doc_code_examples)]
[01:50:50] LL | | /// Some docs.
[01:50:50] LL | | //~^ ERROR Missing code example in this documentation
[01:50:50] ...  |
[01:50:50] LL | |     pub fn bar() {}
---
[01:50:50] 
[01:50:50] error: Missing code example in this documentation
[01:50:50]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:9:1
[01:50:50]    |
[01:50:50] LL | /// And then, the princess died.
[01:50:50] 
[01:50:50] error: Missing code example in this documentation
[01:50:50]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:12:5
[01:50:50]    |
[01:50:50]    |
[01:50:50] LL |     /// Or maybe not because she saved herself!
[01:50:50] 
[01:50:50] error: aborting due to 4 previous errors
[01:50:50] 
[01:50:50] 
[01:50:50] 
[01:50:50] ------------------------------------------
[01:50:50] 
[01:50:50] 
[01:50:50] ---- [ui] rustdoc-ui/lint-missing-doc-code-example.rs stdout ----
[01:50:50] 
[01:50:50] error: /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:19: unexpected error: '19:1: 20:2: Missing code example in this documentation [missing_doc_code_examples]'
[01:50:50] 
[01:50:50] error: /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:37: unexpected error: '37:3: 37:10: Missing code example in this documentation [missing_doc_code_examples]'
[01:50:50] error: 2 unexpected errors found, 0 expected errors not found
[01:50:50] status: exit code: 1
[01:50:50] status: exit code: 1
[01:50:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example/auxiliary"
[01:50:50]     Error {
[01:50:50]         line_num: 19,
[01:50:50]         kind: Some(
[01:50:50]             Error,
[01:50:50]             Error,
[01:50:50]         ),
[01:50:50]         msg: "19:1: 20:2: Missing code example in this documentation [missing_doc_code_examples]",
[01:50:50]     Error {
[01:50:50]         line_num: 37,
[01:50:50]         kind: Some(
[01:50:50]             Error,
[01:50:50]             Error,
[01:50:50]         ),
[01:50:50]         msg: "37:3: 37:10: Missing code example in this documentation [missing_doc_code_examples]",
[01:50:50] ]
[01:50:50] 
[01:50:50] thread '[ui] rustdoc-ui/lint-missing-doc-code-example.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1404:13
[01:50:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:50:50] test result: FAILED. 25 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:50:50] 
[01:50:50] 
[01:50:50] 
[01:50:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:50:50] 
[01:50:50] 
[01:50:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:50:50] Build completed unsuccessfully in 0:38:18
[01:50:50] Build completed unsuccessfully in 0:38:18
[01:50:50] make: *** [check] Error 1
[01:50:50] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:190ecf48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 15:47:11 UTC 2019
---
travis_time:end:178b941b:start=1558108032953093702,finish=1558108032958324680,duration=5230978
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:005ff8da
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d3faff6
travis_time:start:1d3faff6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d69be74
$ dmesg | grep -i kill
