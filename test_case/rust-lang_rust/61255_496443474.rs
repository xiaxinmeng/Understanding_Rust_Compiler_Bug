plain
travis_time:end:0c77d68c:start=1559031758942179067,finish=1559031761124872781,duration=2182693714
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:16:20] ...................................................................................................i 3800/5593
[01:16:24] i................................................................................................... 3900/5593
[01:16:26] ....................i............................................................................... 4000/5593
[01:16:28] ....................................................................................i............... 4100/5593
[01:16:30] .................................F...F.............................................................. 4200/5593
[01:16:46] .................................................................................................... 4400/5593
[01:16:49] .................................................................................................... 4500/5593
[01:16:53] .................................................................................................... 4600/5593
[01:16:57] .................................................................................................... 4700/5593
---
[01:17:36] 1 error: unterminated raw string
[01:17:36] -   --> $DIR/raw-string-long.rs:2:13
[01:17:36] +   --> $DIR/raw-str-long.rs:2:13
[01:17:36] 3    |
[01:17:36] 4 LL |     let a = r##"This
[01:17:36] 5    |             ^^^ unterminated raw string
[01:17:36] 
[01:17:36] The actual stderr differed from the expected stderr.
[01:17:36] The actual stderr differed from the expected stderr.
[01:17:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-str-long/raw-str-long.stderr
[01:17:36] To update references, rerun the tests and pass the `--bless` flag
[01:17:36] To only update this specific test, also pass `--test-args parser/raw/raw-str-long.rs`
[01:17:36] error: 1 errors occurred comparing output.
[01:17:36] status: exit code: 1
[01:17:36] status: exit code: 1
[01:17:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw/raw-str-long.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-str-long" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-str-long/auxiliary" "-A" "unused"
[01:17:36] ------------------------------------------
[01:17:36] 
[01:17:36] ------------------------------------------
[01:17:36] stderr:
[01:17:36] stderr:
[01:17:36] ------------------------------------------
[01:17:36] error: unterminated raw string
[01:17:36]   --> /checkout/src/test/ui/parser/raw/raw-str-long.rs:2:13
[01:17:36]    |
[01:17:36] LL |     let a = r##"This //~ ERROR unterminated raw string
[01:17:36]    |             ^^^ unterminated raw string
[01:17:36] help: Raw string could be meant to end here
[01:17:36] LL |     "##;
[01:17:36]    |     ^^^
[01:17:36]    |     ^^^
[01:17:36] help: Raw string could be meant to end here
[01:17:36] LL | }"##
[01:17:36]    |   ^^^
[01:17:36] 
[01:17:36] error: aborting due to previous error
---
[01:17:36] 1 error: unterminated raw string
[01:17:36] -   --> $DIR/raw_string.rs:2:13
[01:17:36] +   --> $DIR/raw-str.rs:2:13
[01:17:36] 3    |
[01:17:36] 4 LL |     let x = r##"lol"#;
[01:17:36] 5    |             ^^^ unterminated raw string
[01:17:36] 
[01:17:36] The actual stderr differed from the expected stderr.
[01:17:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-str/raw-str.stderr
[01:17:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-str/raw-str.stderr
[01:17:36] To update references, rerun the tests and pass the `--bless` flag
[01:17:36] To only update this specific test, also pass `--test-args parser/raw/raw-str.rs`
[01:17:36] error: 1 errors occurred comparing output.
[01:17:36] status: exit code: 1
[01:17:36] status: exit code: 1
[01:17:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw/raw-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-str" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-str/auxiliary" "-A" "unused"
[01:17:36] ------------------------------------------
[01:17:36] 
[01:17:36] ------------------------------------------
[01:17:36] stderr:
[01:17:36] stderr:
[01:17:36] ------------------------------------------
[01:17:36] error: unterminated raw string
[01:17:36]   --> /checkout/src/test/ui/parser/raw/raw-str.rs:2:13
[01:17:36]    |
[01:17:36] LL |     let x = r##"lol"#;
[01:17:36]    |             ^^^ unterminated raw string
[01:17:36] help: Raw string could be meant to end here
[01:17:36]    |
[01:17:36] LL |     let x = r##"lol"##;
[01:17:36]    |                    ^^^
[01:17:36] help: Raw string could be meant to end here
[01:17:36] LL | }"##
[01:17:36]    |   ^^^
[01:17:36] 
[01:17:36] error: aborting due to previous error
---
[01:17:36] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:17:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:36] 
[01:17:36] 
[01:17:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:36] 
[01:17:36] 
[01:17:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:36] Build completed unsuccessfully in 0:05:02
[01:17:36] Build completed unsuccessfully in 0:05:02
[01:17:36] make: *** [check] Error 1
[01:17:36] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10ca7abe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 28 09:40:28 UTC 2019
