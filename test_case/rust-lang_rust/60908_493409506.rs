plain
travis_time:end:070cb8bc:start=1558083170502891338,finish=1558083274223892979,duration=103721001641
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
[01:21:45] 
[01:21:45] running 143 tests
[01:21:48] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:21:50] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:21:50] 
[01:21:50]  finished in 4.646
[01:21:50] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:52] 
[01:21:52] running 9 tests
[01:21:52] iiiiiiiii
[01:21:52] 
[01:21:52]  finished in 0.154
[01:21:52] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:08] 
[01:22:08] running 122 tests
[01:22:33] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:22:38] .i.i......iii.i.....ii
[01:22:38] 
[01:22:38]  finished in 30.129
[01:22:38] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:46:51] 
[01:46:51] running 27 tests
[01:47:05] .............F..........F..
[01:47:05] failures:
[01:47:05] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:47:05] 
[01:47:05] ---- [ui] rustdoc-ui/doc-without-codeblock.rs stdout ----
[01:47:05] ---- [ui] rustdoc-ui/doc-without-codeblock.rs stdout ----
[01:47:05] diff of stderr:
[01:47:05] 
[01:47:05] 1 error: Missing code example in this documentation
[01:47:05] +   --> $DIR/doc-without-codeblock.rs:3:1
[01:47:05] +    |
[01:47:05] + LL | / #![deny(missing_doc_code_examples)]
[01:47:05] + LL | |
[01:47:05] + LL | | /// Some docs.
[01:47:05] + LL | |
[01:47:05] + ...  |
[01:47:05] + LL | |     pub fn bar() {}
[01:47:05] + LL | | }
[01:47:05] 2    |
[01:47:05] 3 note: lint level defined here
[01:47:05] 4   --> $DIR/doc-without-codeblock.rs:3:9
[01:47:05] 
[01:47:05] 
[01:47:05] 
[01:47:05] The actual stderr differed from the expected stderr.
[01:47:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/doc-without-codeblock.stderr
[01:47:05] To update references, rerun the tests and pass the `--bless` flag
[01:47:05] To only update this specific test, also pass `--test-args doc-without-codeblock.rs`
[01:47:05] error: 1 errors occurred comparing output.
[01:47:05] status: exit code: 1
[01:47:05] status: exit code: 1
[01:47:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-without-codeblock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/auxiliary"
[01:47:05] ------------------------------------------
[01:47:05] 
[01:47:05] ------------------------------------------
[01:47:05] stderr:
[01:47:05] stderr:
[01:47:05] ------------------------------------------
[01:47:05] error: Missing code example in this documentation
[01:47:05]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:3:1
[01:47:05]    |
[01:47:05] LL | / #![deny(missing_doc_code_examples)]
[01:47:05] LL | | /// Some docs.
[01:47:05] LL | | //~^ ERROR Missing code example in this documentation
[01:47:05] ...  |
[01:47:05] LL | |     pub fn bar() {}
---
[01:47:05] 
[01:47:05] error: Missing code example in this documentation
[01:47:05]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:9:1
[01:47:05]    |
[01:47:05] LL | /// And then, the princess died.
[01:47:05] 
[01:47:05] error: Missing code example in this documentation
[01:47:05]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:12:5
[01:47:05]    |
[01:47:05]    |
[01:47:05] LL |     /// Or maybe not because she saved herself!
[01:47:05] 
[01:47:05] error: aborting due to 4 previous errors
[01:47:05] 
[01:47:05] 
---
[01:47:05] normalized stderr:
[01:47:05] error: Missing code example in this documentation
[01:47:05]   --> $DIR/lint-missing-doc-code-example.rs:19:1
[01:47:05]    |
[01:47:05] LL | / mod module1 {
[01:47:05]    | |_^
[01:47:05]    |
[01:47:05] note: lint level defined here
[01:47:05]   --> $DIR/lint-missing-doc-code-example.rs:2:9
---
[01:47:05] 
[01:47:05] 
[01:47:05] 
[01:47:05] The actual stderr differed from the expected stderr.
[01:47:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example/lint-missing-doc-code-example.stderr
[01:47:05] To update references, rerun the tests and pass the `--bless` flag
[01:47:05] To only update this specific test, also pass `--test-args lint-missing-doc-code-example.rs`
[01:47:05] error: 1 errors occurred comparing output.
[01:47:05] status: exit code: 1
[01:47:05] status: exit code: 1
[01:47:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example/auxiliary"
[01:47:05] ------------------------------------------
[01:47:05] 
[01:47:05] ------------------------------------------
[01:47:05] stderr:
[01:47:05] stderr:
[01:47:05] ------------------------------------------
[01:47:05] error: Missing code example in this documentation
[01:47:05]   --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:19:1
[01:47:05]    |
[01:47:05] LL | / mod module1 {
[01:47:05]    | |_^
[01:47:05]    |
[01:47:05] note: lint level defined here
[01:47:05]   --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:2:9
---
[01:47:05] test result: FAILED. 25 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:47:05] 
[01:47:05] 
[01:47:05] 
[01:47:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:47:05] 
[01:47:05] 
[01:47:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:05] Build completed unsuccessfully in 0:37:17
[01:47:05] Build completed unsuccessfully in 0:37:17
[01:47:05] Makefile:48: recipe for target 'check' failed
[01:47:05] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001dc180
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 10:41:49 UTC 2019
---
travis_time:end:24f41cb0:start=1558089710924000297,finish=1558089710929381346,duration=5381049
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3cc035cd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
