plain
travis_time:end:075e2efd:start=1558093230783266399,finish=1558093326652699327,duration=95869432928
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
[01:26:14] 
[01:26:14] running 143 tests
[01:26:17] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:26:19] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:26:19] 
[01:26:19]  finished in 5.078
[01:26:19] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:21] 
[01:26:21] running 9 tests
[01:26:21] iiiiiiiii
[01:26:21] 
[01:26:21]  finished in 0.157
[01:26:21] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:38] 
[01:26:38] running 122 tests
[01:27:05] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:27:11] .i.i......iii.i.....ii
[01:27:11] 
[01:27:11]  finished in 32.371
[01:27:11] travis_fold:end:test_debuginfo

---
[01:52:42] 
[01:52:42] running 27 tests
[01:52:57] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:52:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:52:57] .............F...........F.
[01:52:57] 
[01:52:57] ---- [ui] rustdoc-ui/doc-without-codeblock.rs stdout ----
[01:52:57] diff of stderr:
[01:52:57] 
---
[01:52:57] 13   --> $DIR/doc-without-codeblock.rs:3:9
[01:52:57] 
[01:52:57] 
[01:52:57] The actual stderr differed from the expected stderr.
[01:52:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/doc-without-codeblock.stderr
[01:52:57] To update references, rerun the tests and pass the `--bless` flag
[01:52:57] To only update this specific test, also pass `--test-args doc-without-codeblock.rs`
[01:52:57] error: 1 errors occurred comparing output.
[01:52:57] status: exit code: 1
[01:52:57] status: exit code: 1
[01:52:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-without-codeblock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/auxiliary"
[01:52:57] ------------------------------------------
[01:52:57] 
[01:52:57] ------------------------------------------
[01:52:57] stderr:
[01:52:57] stderr:
[01:52:57] ------------------------------------------
[01:52:57] error: Missing code example in this documentation
[01:52:57]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:3:1
[01:52:57]    |
[01:52:57] LL | / #![deny(missing_doc_code_examples)]
[01:52:57] LL | | /// Some docs.
[01:52:57] LL | | //~^ ERROR Missing code example in this documentation
[01:52:57] ...  |
[01:52:57] LL | |     pub fn bar() {}
---
[01:52:57] 
[01:52:57] error: Missing code example in this documentation
[01:52:57]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:9:1
[01:52:57]    |
[01:52:57] LL | /// And then, the princess died.
[01:52:57] 
[01:52:57] error: Missing code example in this documentation
[01:52:57]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:12:5
[01:52:57]    |
[01:52:57]    |
[01:52:57] LL |     /// Or maybe not because she saved herself!
[01:52:57] 
[01:52:57] error: aborting due to 4 previous errors
[01:52:57] 
[01:52:57] 
---
[01:52:57] 1 error: Missing code example in this documentation
[01:52:57] -   --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:19:1
[01:52:57] +   --> $DIR/lint-missing-doc-code-example.rs:19:1
[01:52:57] 3    |
[01:52:57] 4 LL | / mod module1 {
[01:52:57] + LL | | }
[01:52:57] 6    |
[01:52:57] 7 note: lint level defined here
[01:52:57] 
[01:52:57] -   --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:2:9
---
[01:52:57] 10 
[01:52:57] 
[01:52:57] 
[01:52:57] The actual stderr differed from the expected stderr.
[01:52:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example/lint-missing-doc-code-example.stderr
[01:52:57] To update references, rerun the tests and pass the `--bless` flag
[01:52:57] To only update this specific test, also pass `--test-args lint-missing-doc-code-example.rs`
[01:52:57] error: 1 errors occurred comparing output.
[01:52:57] status: exit code: 1
[01:52:57] status: exit code: 1
[01:52:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example/auxiliary"
[01:52:57] ------------------------------------------
[01:52:57] 
[01:52:57] ------------------------------------------
[01:52:57] stderr:
[01:52:57] stderr:
[01:52:57] ------------------------------------------
[01:52:57] error: Missing code example in this documentation
[01:52:57]   --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:19:1
[01:52:57]    |
[01:52:57] LL | / mod module1 {
[01:52:57]    | |_^
[01:52:57]    |
[01:52:57] note: lint level defined here
[01:52:57]   --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:2:9
---
[01:52:57] test result: FAILED. 25 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:52:57] 
[01:52:57] 
[01:52:57] 
[01:52:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:52:57] 
[01:52:57] 
[01:52:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:52:57] Build completed unsuccessfully in 0:39:20
[01:52:57] Build completed unsuccessfully in 0:39:20
[01:52:57] make: *** [check] Error 1
[01:52:57] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:295b4751
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 13:35:14 UTC 2019
---
travis_time:end:011c7c84:start=1558100115886230800,finish=1558100115945462228,duration=59231428
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f687617
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:116b59c0
$ dmesg | grep -i kill
