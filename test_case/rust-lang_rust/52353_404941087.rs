plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:56] 
[00:52:56] running 90 tests
[00:53:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:53:09] .........................................................F................................
[00:53:09] 
[00:53:09] ---- [incremental] incremental/issue-49595/issue_49595.rs stdout ----
[00:53:09] 
[00:53:09] 
[00:53:09] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:53:09] status: exit code: 101
[00:53:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue_49595.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/issue_49595.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/auxiliary"
[00:53:09] ------------------------------------------
[00:53:09] 
[00:53:09] ------------------------------------------
[00:53:09] stderr:
[00:53:09] stderr:
[00:53:09] ------------------------------------------
[00:53:09] {"message":"expected module named `issue_49595-tests` to be Codegened but is Reused","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/issue-49595/issue_49595.rs","byte_start":612,"byte_end":683,"line_start":18,"line_end":18,"column_start":1,"column_end":72,"is_primary":true,"text":[{"text":"#![rustc_partition_codegened(module=\"issue_49595-tests\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":72}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected module named `issue_49595-tests` to be Codegened but is Reused\n  --> /checkout/src/test/incremental/issue-49595/issue_49595.rs:18:1\n   |\nLL | #![rustc_partition_codegened(module=\"issue_49595-tests\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:53:09] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:53:09] ------------------------------------------
[00:53:09] 
[00:53:09] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:53:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:53:09] test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:53:09] 
[00:53:09] 
[00:53:09] 
[00:53:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-hFri, 13 Jul 2018 20:11:44 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
---
travis_time:end:0031f296:start=1531512706512068392,finish=1531512706520219355,duration=8150963
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19b13b27
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a1cdef0
$ dmesg | grep -i kill
