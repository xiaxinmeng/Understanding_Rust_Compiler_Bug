plain
travis_time:end:04a29704:start=1554633994007961660,finish=1554633994987705499,duration=979743839
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:03:29] .................................................................................................... 1400/5527
[01:03:33] .................................................................................................... 1500/5527
[01:03:36] .................................................................................................... 1600/5527
[01:03:39] .......................................................i............................................ 1700/5527
[01:03:43] .......................................F............................................................ 1800/5527
[01:03:51] .................................................................................................... 2000/5527
[01:03:54] ...........................................................................................i........ 2100/5527
[01:03:54] ...........................................................................................i........ 2100/5527
[01:03:58] .......................................F............................................................ 2200/5527
[01:04:07] .................................................................................................... 2400/5527
[01:04:12] .................................................................................................... 2500/5527
[01:04:15] .................................................................................................... 2600/5527
[01:04:19] .................................................................................................... 2700/5527
---
[01:06:17] ---- [ui] ui/duplicate/dupe-symbols-7.rs stdout ----
[01:06:17] diff of stderr:
[01:06:17] 
[01:06:17] 6    |
[01:06:17] 7    = help: did you use #[no_mangle] on `fn main`? Use #[start] instead
[01:06:17] 8 
[01:06:17] - thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:06:17] - note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:17] 12 
[01:06:17] 13 
[01:06:17] 
[01:06:17] 
[01:06:17] 
[01:06:17] The actual stderr differed from the expected stderr.
[01:06:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-7/dupe-symbols-7.stderr
[01:06:17] To update references, rerun the tests and pass the `--bless` flag
[01:06:17] To only update this specific test, also pass `--test-args duplicate/dupe-symbols-7.rs`
[01:06:17] error: 1 errors occurred comparing output.
[01:06:17] status: exit code: 1
[01:06:17] status: exit code: 1
[01:06:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate/dupe-symbols-7.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-7/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/dupe-symbols-7/auxiliary" "-A" "unused"
[01:06:17] ------------------------------------------
[01:06:17] 
[01:06:17] ------------------------------------------
[01:06:17] stderr:
[01:06:17] stderr:
[01:06:17] ------------------------------------------
[01:06:17] {"message":"entry symbol `main` defined multiple times","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/duplicate/dupe-symbols-7.rs","byte_start":98,"byte_end":109,"line_start":6,"line_end":6,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"fn main(){}","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you use #[no_mangle] on `fn main`? Use #[start] instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: entry symbol `main` defined multiple times\n  --> /checkout/src/test/ui/duplicate/dupe-symbols-7.rs:6:1\n   |\nLL | fn main(){}\n   | ^^^^^^^^^^^\n   |\n   = help: did you use #[no_mangle] on `fn main`? Use #[start] instead\n\n"}
[01:06:17] 
[01:06:17] ------------------------------------------
[01:06:17] 
[01:06:17] thread '[ui] ui/duplicate/dupe-symbols-7.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:06:17] thread '[ui] ui/duplicate/dupe-symbols-7.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:06:17] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:17] 
[01:06:17] ---- [ui] ui/huge-array.rs stdout ----
[01:06:17] diff of stderr:
[01:06:17] 
[01:06:17] 1 error: the type `[[u8; 1518599999]; 1518600000]` is too big for the current architecture
[01:06:17] 2 
[01:06:17] + thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:06:17] + note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:17] 4 
[01:06:17] 5 
[01:06:17] 
[01:06:17] 
[01:06:17] 
[01:06:17] The actual stderr differed from the expected stderr.
[01:06:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-array/huge-array.stderr
[01:06:17] To update references, rerun the tests and pass the `--bless` flag
[01:06:17] To only update this specific test, also pass `--test-args huge-array.rs`
[01:06:17] error: 1 errors occurred comparing output.
[01:06:17] status: exit code: 1
[01:06:17] status: exit code: 1
[01:06:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/huge-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-array/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-array/auxiliary" "-A" "unused"
[01:06:17] ------------------------------------------
[01:06:17] 
[01:06:17] ------------------------------------------
[01:06:17] stderr:
[01:06:17] stderr:
[01:06:17] ------------------------------------------
[01:06:17] {"message":"the type `[[u8; 1518599999]; 1518600000]` is too big for the current architecture","code":null,"level":"error","spans":[],"children":[],"rendered":"error: the type `[[u8; 1518599999]; 1518600000]` is too big for the current architecture\n\n"}
[01:06:17] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:06:17] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:17] 
[01:06:17] ------------------------------------------
[01:06:17] 
[01:06:17] 
[01:06:17] thread '[ui] ui/huge-array.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:06:17] 
[01:06:17] ---- [ui] ui/issues/issue-17913.rs stdout ----
[01:06:17] diff of stderr:
[01:06:17] 
[01:06:17] 1 error: the type `[&usize; N]` is too big for the current architecture
[01:06:17] 2 
[01:06:17] + thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:06:17] + note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:17] 4 
[01:06:17] 5 
[01:06:17] 
[01:06:17] 
[01:06:17] 
[01:06:17] The actual stderr differed from the expected stderr.
[01:06:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17913/issue-17913.stderr
[01:06:17] To update references, rerun the tests and pass the `--bless` flag
[01:06:17] To only update this specific test, also pass `--test-args issues/issue-17913.rs`
[01:06:17] error: 1 errors occurred comparing output.
[01:06:17] status: exit code: 1
[01:06:17] status: exit code: 1
[01:06:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17913.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17913/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17913/auxiliary" "-A" "unused"
[01:06:17] ------------------------------------------
[01:06:17] 
[01:06:17] ------------------------------------------
[01:06:17] stderr:
[01:06:17] stderr:
[01:06:17] ------------------------------------------
[01:06:17] {"message":"the type `[&usize; 17293822569102704640]` is too big for the current architecture","code":null,"level":"error","spans":[],"children":[],"rendered":"error: the type `[&usize; 17293822569102704640]` is too big for the current architecture\n\n"}
[01:06:17] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:06:17] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:06:17] 
[01:06:17] ------------------------------------------
[01:06:17] 
---
[01:06:17] 
[01:06:17] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:06:17] 
[01:06:17] 
[01:06:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:17] 
[01:06:17] 
[01:06:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:17] Build completed unsuccessfully in 0:04:38
[01:06:17] Build completed unsuccessfully in 0:04:38
[01:06:17] make: *** [check] Error 1
[01:06:17] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ff76ea4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr  7 11:53:03 UTC 2019
---
travis_time:end:01e55fbd:start=1554637985504485760,finish=1554637985509690247,duration=5204487
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2777cc1c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cb80176
travis_time:start:0cb80176
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:29924e68
$ dmesg | grep -i kill
