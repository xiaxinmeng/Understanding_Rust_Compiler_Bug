plain
travis_time:end:059a92ca:start=1554923787358649464,finish=1554923865665391512,duration=78306742048
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:06:00] .................................................................................................... 4800/5531
[01:06:03] .................................................................................................... 4900/5531
[01:06:06] .................................................................................................... 5000/5531
[01:06:11] .................................................................................................... 5100/5531
[01:06:14] ...........................................................................F........................ 5200/5531
[01:06:21] .................................................................................................... 5400/5531
[01:06:24] .....................................................................i.............................. 5500/5531
[01:06:25] ...............................
[01:06:25] failures:
[01:06:25] failures:
[01:06:25] 
[01:06:25] ---- [ui] ui/try-block/try-block-catch.rs stdout ----
[01:06:25] diff of stderr:
[01:06:25] 
[01:06:25] 1 error: `try {} catch` is not a valid syntax
[01:06:25] 2   --> $DIR/try-block-catch.rs:8:7
[01:06:25] 3    |
[01:06:25] - LL |     } catch { }; //~ ERROR `try {} catch` is not a valid syntax
[01:06:25] + LL |     } catch { };
[01:06:25] 6    |
[01:06:25] 6    |
[01:06:25] 7    = help: try using `match` on the result of the `try` block instead
[01:06:25] 
[01:06:25] The actual stderr differed from the expected stderr.
[01:06:25] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/try-block-catch.stderr
[01:06:25] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/try-block-catch.stderr
[01:06:25] To update references, rerun the tests and pass the `--bless` flag
[01:06:25] To only update this specific test, also pass `--test-args try-block/try-block-catch.rs`
[01:06:25] error: 1 errors occurred comparing output.
[01:06:25] status: exit code: 1
[01:06:25] status: exit code: 1
[01:06:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-catch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-catch/auxiliary" "-A" "unused"
[01:06:25] ------------------------------------------
[01:06:25] 
[01:06:25] ------------------------------------------
[01:06:25] stderr:
[01:06:25] stderr:
[01:06:25] ------------------------------------------
[01:06:25] {"message":"`try {} catch` is not a valid syntax","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/try-block/try-block-catch.rs","byte_start":124,"byte_end":129,"line_start":8,"line_end":8,"column_start":7,"column_end":12,"is_primary":true,"text":[{"text":"    } catch { }; //~ ERROR `try {} catch` is not a valid syntax","highlight_start":7,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using `match` on the result of the `try` block instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `try {} catch` is not a valid syntax\n  --> /checkout/src/test/ui/try-block/try-block-catch.rs:8:7\n   |\nLL |     } catch { }; //~ ERROR `try {} catch` is not a valid syntax\n   |       ^^^^^\n   |\n   = help: try using `match` on the result of the `try` block instead\n\n"}
[01:06:25] 
[01:06:25] ------------------------------------------
[01:06:25] 
[01:06:25] thread '[ui] ui/try-block/try-block-catch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:06:25] 
[01:06:25] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:06:25] 
[01:06:25] 
[01:06:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:25] 
[01:06:25] 
[01:06:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:25] Build completed unsuccessfully in 0:04:21
[01:06:25] Build completed unsuccessfully in 0:04:21
[01:06:25] make: *** [check] Error 1
[01:06:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01e89238
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 20:24:20 UTC 2019
---
travis_time:end:0bef3624:start=1554927861563602999,finish=1554927861571259717,duration=7656718
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:048cfc5a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b488680
$ dmesg | grep -i kill
