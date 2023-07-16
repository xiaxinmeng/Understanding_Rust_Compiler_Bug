plain
travis_time:end:0a0748bc:start=1561416508805736044,finish=1561416599369210419,duration=90563474375
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    98% |███████████████████████████████▋| 1.7MB 48.9MB/s eta 0:00:01
    99% |███████████████████████████████▉| 1.7MB 48.3MB/s eta 0:00:01
    100% |████████████████████████████████| 1.7MB 10.1MB/s 
Collecting botocore==1.12.175 (from awscli)
  Downloading https://files.pythonhosted.org/packages/19/ff/fff69109c7f4f97f393b0b948eab16caf3464204fe5cf1955d9d1e1879fa/botocore-1.12.175-py2.py3-none-any.whl (5.6MB)
    0% |▏                               | 20kB 22.9MB/s eta 0:00:01
    0% |▏                               | 30kB 28.4MB/s eta 0:00:01
    0% |▎                               | 40kB 31.0MB/s eta 0:00:01
    0% |▎                               | 51kB 32.9MB/s eta 0:00:01
---
[00:57:40] .................................................................................................... 3000/5698
[00:57:44] .................................................................................................... 3100/5698
[00:57:48] .................................................................................................... 3200/5698
[00:57:51] .................................................................................................... 3300/5698
[00:57:55] .....................F.............................................................................. 3400/5698
[00:58:02] .............................................................................ii...i..ii............. 3600/5698
[00:58:06] .................................................................................................... 3700/5698
[00:58:10] .................................................................................................... 3800/5698
[00:58:13] .........................................................................................ii......... 3900/5698
---
[00:59:28] 29    |         ^ help: consider prefixing with an underscore: `_b`
[00:59:28] 
[00:59:28] 
[00:59:28] The actual stderr differed from the expected stderr.
[00:59:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-variables/lint-unused-variables.stderr
[00:59:28] To update references, rerun the tests and pass the `--bless` flag
[00:59:28] To only update this specific test, also pass `--test-args lint/lint-unused-variables.rs`
[00:59:28] error: 1 errors occurred comparing output.
[00:59:28] status: exit code: 1
[00:59:28] status: exit code: 1
[00:59:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unused-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-variables" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-variables/auxiliary" "-A" "unused"
[00:59:28] ------------------------------------------
[00:59:28] 
[00:59:28] ------------------------------------------
[00:59:28] stderr:
---
[00:59:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:59:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:28] 
[00:59:28] 
[00:59:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:28] 
[00:59:28] 
[00:59:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:28] Build completed unsuccessfully in 0:55:15
---
travis_time:end:03fd4391:start=1561420179079289169,finish=1561420179084376746,duration=5087577
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e901916
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07208377
travis_time:start:07208377
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0474631e
$ dmesg | grep -i kill
