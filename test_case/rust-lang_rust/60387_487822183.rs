plain
travis_time:end:15839499:start=1556595333876738337,finish=1556595419273925913,duration=85397187576
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
[01:20:11] 
[01:20:11] running 9 tests
[01:20:11] iiiiiiiii
[01:20:11] 
[01:20:11]  finished in 0.154
[01:20:11] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:27] 
[01:20:27] running 121 tests
[01:20:52] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:20:56] i.i......iii.i.....ii
[01:20:56] 
[01:20:56]  finished in 29.948
[01:20:56] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:59] 
[01:24:59] running 305 tests
[01:26:09] ............F...............i...F.....FF............................................................ 100/305
[01:27:08] ................F................F...F.F...i.........F....................F.....F...............F... 200/305
[01:28:09] ...F.................................F.........................................FF................... 300/305
[01:28:12] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:28:12] .....
[01:28:12] failures:
[01:28:12] 
---
[01:28:12] ---- [rustdoc] rustdoc/doctest-manual-crate-name.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name" "/checkout/src/test/rustdoc/doctest-manual-crate-name.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/edition-flag.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag" "/checkout/src/test/rustdoc/edition-flag.rs" "--test" "-Z" "unstable-options" "--edition=2018"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-18199.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199" "/checkout/src/test/rustdoc/issue-18199.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-23106.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-23744.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744" "/checkout/src/test/rustdoc/issue-23744.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-25944.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-30252.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-38129.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-43153.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/issue-48377.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377" "/checkout/src/test/rustdoc/issue-48377.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/process-termination.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] ---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----
[01:28:12] 
[01:28:12] error: rustdoc failed!
[01:28:12] status: exit code: 1
[01:28:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
[01:28:12] ------------------------------------------
[01:28:12] 
[01:28:12] ------------------------------------------
[01:28:12] stderr:
---
[01:28:12] test result: FAILED. 287 passed; 16 failed; 2 ignored; 0 measured; 0 filtered out
[01:28:12] 
[01:28:12] 
[01:28:12] 
[01:28:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:28:12] 
[01:28:12] 
[01:28:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:12] Build completed unsuccessfully in 0:19:43
[01:28:12] Build completed unsuccessfully in 0:19:43
[01:28:12] Makefile:48: recipe for target 'check' failed
[01:28:12] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dab7055
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 05:05:21 UTC 2019
---
travis_time:end:0d88f700:start=1556600723557060129,finish=1556600723561944083,duration=4883954
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2afe235a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2ccfbdad
travis_time:start:2ccfbdad
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:30eeb2b2
$ dmesg | grep -i kill
