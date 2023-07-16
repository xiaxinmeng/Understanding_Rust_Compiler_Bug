plain
travis_time:end:2d9c83f6:start=1549189075181025206,finish=1549189155925707163,duration=80744681957
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:56:23] .................................................................................................... 2900/5362
[00:56:26] .................................................................................................... 3000/5362
[00:56:29] .................................................................................................... 3100/5362
[00:56:33] .................................................................................................... 3200/5362
[00:56:36] .........F.........i................................................................................ 3300/5362
[00:56:42] .................................................................................................... 3500/5362
[00:56:46] .................................................................................................... 3600/5362
[00:56:48] ..................................................................................ii................ 3700/5362
[00:56:51] .................................................................................................... 3800/5362
---
[00:57:46] 
[00:57:46] ---- [ui] ui/macros/macro-at-most-once-rep-2015-ques-rep.rs stdout ----
[00:57:46] diff of stderr:
[00:57:46] 
[00:57:46] 12 LL |     ($(a),?) => {} //~ERROR expected `*` or `+`
[00:57:46] 14    |
[00:57:46] -    = note: `?` is not a macro repetition operator in the 2015 edition, but is accepted in the 2018 edition
[00:57:46] -    = note: `?` is not a macro repetition operator in the 2015 edition, but is accepted in the 2018 edition
[00:57:46] +    = note: `?` is not a macro repetition operator in the 2015 edition,but is accepted in the 2018 edition
[00:57:46] 17 error: aborting due to 2 previous errors
[00:57:46] 18 
[00:57:46] 
[00:57:46] 
[00:57:46] 
[00:57:46] The actual stderr differed from the expected stderr.
[00:57:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-2015-ques-rep/macro-at-most-once-rep-2015-ques-rep.stderr
[00:57:46] To update references, rerun the tests and pass the `--bless` flag
[00:57:46] To only update this specific test, also pass `--test-args macros/macro-at-most-once-rep-2015-ques-rep.rs`
[00:57:46] error: 1 errors occurred comparing output.
[00:57:46] status: exit code: 1
[00:57:46] status: exit code: 1
[00:57:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-at-most-once-rep-2015-ques-rep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-2015-ques-rep/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-2015-ques-rep/auxiliary" "-A" "unused"
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] stderr:
[00:57:46] stderr:
[00:57:46] ------------------------------------------
[00:57:46] {"message":"expected `*` or `+`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2015-ques-rep.rs","byte_start":138,"byte_end":139,"line_start":6,"line_end":6,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    ($(a)?) => {} //~ERROR expected `*` or `+`","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`?` is not a macro repetition operator in the 2015 edition, but is accepted in the 2018 edition","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: expected `*` or `+`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2015-ques-rep.rs:6:10\n   |\nLL |     ($(a)?) => {} //~ERROR expected `*` or `+`\n   |          ^\n   |\n   = note: `?` is not a macro repetition operator in the 2015 edition, but is accepted in the 2018 edition\n\n"}
[00:57:46] {"message":"expected `*` or `+`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2015-ques-rep.rs","byte_start":208,"byte_end":209,"line_start":10,"line_end":10,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    ($(a),?) => {} //~ERROR expected `*` or `+`","highlight_start":11,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`?` is not a macro repetition operator in the 2015 edition,but is accepted in the 2018 edition","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: expected `*` or `+`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2015-ques-rep.rs:10:11\n   |\nLL |     ($(a),?) => {} //~ERROR expected `*` or `+`\n   |           ^\n   |\n   = note: `?` is not a macro repetition operator in the 2015 edition,but is accepted in the 2018 edition\n\n"}
[00:57:46] 
[00:57:46] ------------------------------------------
[00:57:46] 
[00:57:46] thread '[ui] ui/macros/macro-at-most-once-rep-2015-ques-rep.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[00:57:46] 
[00:57:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[00:57:46] 
[00:57:46] 
[00:57:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:46] 
[00:57:46] 
[00:57:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:46] Build completed unsuccessfully in 0:03:53
[00:57:46] Build completed unsuccessfully in 0:03:53
[00:57:46] make: *** [check] Error 1
[00:57:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b6ade4a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 11:17:10 UTC 2019
---
travis_time:end:07927e33:start=1549192631263599608,finish=1549192631267688379,duration=4088771
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f2ececc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0146f1f8
travis_time:start:0146f1f8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2c55ed2a
$ dmesg | grep -i kill
