plain
travis_time:end:12e4f71a:start=1554648403791901503,finish=1554648406142912939,duration=2351011436
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:09:29] .................................................................................................... 2800/5527
[01:09:33] .................................................................................................... 2900/5527
[01:09:38] .................................................................................................... 3000/5527
[01:09:42] .................................................................................................... 3100/5527
[01:09:46] ............................................................F....................................... 3200/5527
[01:09:55] ...........................i........................................................................ 3400/5527
[01:09:59] .................................................................................................... 3500/5527
[01:10:03] .ii...i..ii......................................................................................... 3600/5527
[01:10:08] .................................................................................................... 3700/5527
---
[01:11:28] 
[01:11:28] ---- [ui] ui/linkage3.rs stdout ----
[01:11:28] diff of stderr:
[01:11:28] 
[01:11:28] 4 LL |     #[linkage = "foo"] static foo: *const i32;
[01:11:28] 6 
[01:11:28] 6 
[01:11:28] + thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:11:28] + note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:28] 8 
[01:11:28] 9 
[01:11:28] 
[01:11:28] 
[01:11:28] 
[01:11:28] The actual stderr differed from the expected stderr.
[01:11:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage3/linkage3.stderr
[01:11:28] To update references, rerun the tests and pass the `--bless` flag
[01:11:28] To only update this specific test, also pass `--test-args linkage3.rs`
[01:11:28] error: 1 errors occurred comparing output.
[01:11:28] status: exit code: 1
[01:11:28] status: exit code: 1
[01:11:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage3/auxiliary" "-A" "unused"
[01:11:28] ------------------------------------------
[01:11:28] 
[01:11:28] ------------------------------------------
[01:11:28] stderr:
[01:11:28] stderr:
[01:11:28] ------------------------------------------
[01:11:28] {"message":"invalid linkage specified","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/linkage3.rs","byte_start":54,"byte_end":77,"line_start":4,"line_end":4,"column_start":24,"column_end":47,"is_primary":true,"text":[{"text":"    #[linkage = \"foo\"] static foo: *const i32;","highlight_start":24,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: invalid linkage specified\n  --> /checkout/src/test/ui/linkage3.rs:4:24\n   |\nLL |     #[linkage = \"foo\"] static foo: *const i32;\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:28] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:11:28] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:11:28] 
[01:11:28] ------------------------------------------
[01:11:28] 
---
[01:11:28] 
[01:11:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:11:28] 
[01:11:28] 
[01:11:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:28] 
[01:11:28] 
[01:11:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:28] Build completed unsuccessfully in 0:04:56
[01:11:28] Build completed unsuccessfully in 0:04:56
[01:11:28] make: *** [check] Error 1
[01:11:28] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09114b9a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr  7 15:58:26 UTC 2019
---
travis_time:end:16531dc9:start=1554652708151519886,finish=1554652708157313802,duration=5793916
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04ecdf90
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05e861da
travis_time:start:05e861da
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0130713c
$ dmesg | grep -i kill
