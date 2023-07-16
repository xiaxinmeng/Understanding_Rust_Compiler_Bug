plain
travis_time:end:1486f39e:start=1556581849530958173,finish=1556581935702154831,duration=86171196658
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
[01:22:50] 
[01:22:50] running 9 tests
[01:22:50] iiiiiiiii
[01:22:50] 
[01:22:50]  finished in 0.155
[01:22:50] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:07] 
[01:23:07] running 121 tests
[01:23:32] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:23:37] i.i......iii.i.....ii
[01:23:37] 
[01:23:37]  finished in 30.550
[01:23:37] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:27:45] 
[01:27:45] running 305 tests
[01:28:56] ............F...............i...F....FF............................................................. 100/305
[01:29:57] ................F................F..F..F...i........F.....................F.....F................F.. 200/305
[01:31:00] ...F.................................F.........................................FF................... 300/305
[01:31:03] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:31:03] .....
[01:31:03] failures:
[01:31:03] 
---
[01:31:03] ---- [rustdoc] rustdoc/doctest-manual-crate-name.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name" "/checkout/src/test/rustdoc/doctest-manual-crate-name.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/edition-flag.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag" "/checkout/src/test/rustdoc/edition-flag.rs" "--test" "-Z" "unstable-options" "--edition=2018"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-18199.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199" "/checkout/src/test/rustdoc/issue-18199.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-23744.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744" "/checkout/src/test/rustdoc/issue-23744.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-43153.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/issue-48377.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377" "/checkout/src/test/rustdoc/issue-48377.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/process-termination.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:31:03] 
[01:31:03] error: rustdoc failed!
[01:31:03] status: exit code: 1
[01:31:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:31:03] ------------------------------------------
[01:31:03] 
[01:31:03] ------------------------------------------
[01:31:03] stderr:
---
[01:31:03] test result: FAILED. 287 passed; 16 failed; 2 ignored; 0 measured; 0 filtered out
[01:31:03] 
[01:31:03] 
[01:31:03] 
[01:31:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:31:03] 
[01:31:03] 
[01:31:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:03] Build completed unsuccessfully in 0:20:22
[01:31:03] Build completed unsuccessfully in 0:20:22
[01:31:03] make: *** [check] Error 1
[01:31:03] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e5380e8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 01:23:28 UTC 2019
---
travis_time:end:02952ba6:start=1556587410721652229,finish=1556587410788724794,duration=67072565
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:060b781c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0755277b
$ dmesg | grep -i kill
