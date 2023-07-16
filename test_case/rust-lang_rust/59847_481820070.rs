plain
travis_time:end:0a0904ec:start=1554918550541482494,finish=1554918623685650324,duration=73144167830
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:07:44] .................................................................................................... 4800/5531
[01:07:48] .................................................................................................... 4900/5531
[01:07:52] .................................................................................................... 5000/5531
[01:07:57] .................................................................................................... 5100/5531
[01:08:00] ...........................................................................F........................ 5200/5531
[01:08:08] .................................................................................................... 5400/5531
[01:08:11] .....................................................................i.............................. 5500/5531
[01:08:12] ...............................
[01:08:12] failures:
[01:08:12] failures:
[01:08:12] 
[01:08:12] ---- [ui] ui/try-block/try-block-catch.rs stdout ----
[01:08:12] diff of stderr:
[01:08:12] 
[01:08:12] 1 error: `try {} catch` is not a valid syntax
[01:08:12] -   --> $DIR/try-block-catch.rs:8:4
[01:08:12] +   --> $DIR/try-block-catch.rs:8:7
[01:08:12] 3    |
[01:08:12] - LL |     } catch { }; //~ ERROR `try {} catch` is not a valid syntax
[01:08:12] + LL |     } catch { };
[01:08:12] 6    |
[01:08:12] 6    |
[01:08:12] 7    = help: try using `match` on the result of the `try` block instead
[01:08:12] 
[01:08:12] The actual stderr differed from the expected stderr.
[01:08:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/try-block-catch.stderr
[01:08:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/try-block-catch.stderr
[01:08:12] To update references, rerun the tests and pass the `--bless` flag
[01:08:12] To only update this specific test, also pass `--test-args try-block/try-block-catch.rs`
[01:08:12] error: 1 errors occurred comparing output.
[01:08:12] status: exit code: 1
[01:08:12] status: exit code: 1
[01:08:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-catch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/auxiliary" "-A" "unused"
[01:08:12] ------------------------------------------
[01:08:12] 
[01:08:12] ------------------------------------------
[01:08:12] stderr:
[01:08:12] stderr:
[01:08:12] ------------------------------------------
[01:08:12] {"message":"`try {} catch` is not a valid syntax","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/try-block/try-block-catch.rs","byte_start":124,"byte_end":129,"line_start":8,"line_end":8,"column_start":7,"column_end":12,"is_primary":true,"text":[{"text":"    } catch { }; //~ ERROR `try {} catch` is not a valid syntax","highlight_start":7,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using `match` on the result of the `try` block instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `try {} catch` is not a valid syntax\n  --> /checkout/src/test/ui/try-block/try-block-catch.rs:8:7\n   |\nLL |     } catch { }; //~ ERROR `try {} catch` is not a valid syntax\n   |       ^^^^^\n   |\n   = help: try using `match` on the result of the `try` block instead\n\n"}
[01:08:12] 
[01:08:12] ------------------------------------------
[01:08:12] 
[01:08:12] thread '[ui] ui/try-block/try-block-catch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:08:12] 
[01:08:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:08:12] 
[01:08:12] 
[01:08:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:12] 
[01:08:12] 
[01:08:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:12] Build completed unsuccessfully in 0:04:46
[01:08:12] Build completed unsuccessfully in 0:04:46
[01:08:12] make: *** [check] Error 1
[01:08:12] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003c9254
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 18:58:46 UTC 2019
---
travis_time:end:0c35c706:start=1554922728474844431,finish=1554922728480495949,duration=5651518
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e1e06e5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:093d47e4
travis_time:start:093d47e4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01fdecc8
$ dmesg | grep -i kill
