plain
travis_time:end:0956d68d:start=1552563183397240459,finish=1552563187183073444,duration=3785832985
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:16:23] 
[01:16:23] running 2957 tests
[01:16:37] .................................................................................................... 100/2957
[01:16:50] .................................................................................i.................. 200/2957
[01:17:01] .......................................................................................F............ 300/2957
[01:17:24] .................................................................................................... 500/2957
[01:17:36] .................................................................................................... 600/2957
[01:17:54] .................................................................................................... 700/2957
[01:18:07] .................................................................................................... 800/2957
---
[01:20:17] .................................................................................................... 1800/2957
[01:20:29] .................................................................................................... 1900/2957
[01:20:44] .......i......................................................................i..................... 2000/2957
[01:21:11] .................................................................................................... 2100/2957
[01:21:31] ............................................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:21:36] ...F.................................... 2200/2957
[01:22:08] ...............................................ii................................................... 2400/2957
[01:22:24] .................................................................................................... 2500/2957
[01:22:51] .................................................................................................... 2600/2957
[01:23:11] .................................................................................................... 2700/2957
[01:23:11] .................................................................................................... 2700/2957
[01:23:23] .................................................................................................... 2800/2957
[01:23:35] .................................................................................................... 2900/2957
[01:23:44] .........................................................
[01:23:44] failures:
[01:23:44] 
[01:23:44] ---- [run-pass] run-pass/binops.rs stdout ----
[01:23:44] 
[01:23:44] error: test compilation failed although it shouldn't!
[01:23:44] status: exit code: 1
[01:23:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/binops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binops/auxiliary"
[01:23:44] ------------------------------------------
[01:23:44] 
[01:23:44] ------------------------------------------
[01:23:44] stderr:
[01:23:44] stderr:
[01:23:44] ------------------------------------------
[01:23:44] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/binops.rs","byte_start":121,"byte_end":121,"line_start":6,"line_end":6,"column_start":27,"column_end":27,"is_primary":true,"text":[{"text":"    assert_ne!((!((), ())));","highlight_start":27,"highlight_end":27}],"label":"missing tokens in macro arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/run-pass/binops.rs:6:27\n   |\nLL |     assert_ne!((!((), ())));\n   |                           ^ missing tokens in macro arguments\n\n"}
[01:23:44] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/binops.rs","byte_start":1722,"byte_end":1722,"line_start":77,"line_end":77,"column_start":24,"column_end":24,"is_primary":true,"text":[{"text":"  assert_ne!((r.y, q.y));","highlight_start":24,"highlight_end":24}],"label":"missing tokens in macro arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/run-pass/binops.rs:77:24\n   |\nLL |   assert_ne!((r.y, q.y));\n   |                        ^ missing tokens in macro arguments\n\n"}
[01:23:44] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/binops.rs","byte_start":1767,"byte_end":1767,"line_start":79,"line_end":79,"column_start":20,"column_end":20,"is_primary":true,"text":[{"text":"  assert_ne!((q, r));","highlight_start":20,"highlight_end":20}],"label":"missing tokens in macro arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/run-pass/binops.rs:79:20\n   |\nLL |   assert_ne!((q, r));\n   |                    ^ missing tokens in macro arguments\n\n"}
[01:23:44] 
[01:23:44] ------------------------------------------
[01:23:44] 
[01:23:44] thread '[run-pass] run-pass/binops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:23:44] thread '[run-pass] run-pass/binops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:23:44] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:23:44] 
[01:23:44] ---- [run-pass] run-pass/raw-fat-ptr.rs stdout ----
[01:23:44] 
[01:23:44] error: test compilation failed although it shouldn't!
[01:23:44] status: exit code: 1
[01:23:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/raw-fat-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/raw-fat-ptr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/raw-fat-ptr/auxiliary"
[01:23:44] ------------------------------------------
[01:23:44] 
[01:23:44] ------------------------------------------
[01:23:44] stderr:
[01:23:44] stderr:
[01:23:44] ------------------------------------------
[01:23:44] {"message":"expected expression, found `,`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/raw-fat-ptr.rs","byte_start":285,"byte_end":285,"line_start":11,"line_end":11,"column_start":29,"column_end":29,"is_primary":true,"text":[{"text":"                assert_eq!(!(a[i], a[j]));","highlight_start":29,"highlight_end":29}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `,`\n  --> /checkout/src/test/run-pass/raw-fat-ptr.rs:11:29\n   |\nLL |                 assert_eq!(!(a[i], a[j]));\n   |                             ^ expected expression\n\n"}
[01:23:44] 
[01:23:44] ------------------------------------------
[01:23:44] 
[01:23:44] thread '[run-pass] run-pass/raw-fat-ptr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:23:44] 
[01:23:44] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:23:44] 
[01:23:44] 
[01:23:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:44] 
[01:23:44] 
[01:23:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:44] Build completed unsuccessfully in 0:12:09
[01:23:44] Build completed unsuccessfully in 0:12:09
[01:23:44] Makefile:48: recipe for target 'check' failed
[01:23:44] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:071e25cd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 14 12:57:03 UTC 2019
---
travis_time:end:03301551:start=1552568225246483047,finish=1552568225251396112,duration=4913065
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05feb096
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:154da6e3
travis_time:start:154da6e3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:013217e8
$ dmesg | grep -i kill
