plain
travis_time:end:0129de10:start=1551907100335208293,finish=1551907209384527302,duration=109049319009
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
    99% |████████████████████████████████| 542kB 74.0MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 28.9MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.109 (from awscli)
  Downloading https://files.pythonhosted.org/packages/df/fb/c489f16e723e71b77718229c021e2571009dee33fa2f9abccefcfca93f3f/botocore-1.12.109-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 19.0MB/s eta 0:00:01
    0% |▏                               | 30kB 24.4MB/s eta 0:00:01
    0% |▎                               | 40kB 28.0MB/s eta 0:00:01
    0% |▎                               | 51kB 31.2MB/s eta 0:00:01
---
[01:07:32] .................................................................................................... 300/5428
[01:07:35] .................................................................................................... 400/5428
[01:07:38] .................................................................................................... 500/5428
[01:07:42] ....................................i............................................................... 600/5428
[01:07:46] ............................................................................................F....... 700/5428
[01:07:56] .........................................................................................i.......... 900/5428
[01:08:00] .....i.............................................................................................. 1000/5428
[01:08:04] ..................iiiii............................................................................. 1100/5428
[01:08:07] .................................................................................................... 1200/5428
---
[01:10:48] failures:
[01:10:48] 
[01:10:48] ---- [ui] ui/conditional-compilation/cfg-attr-syntax-validation.rs stdout ----
[01:10:48] 
[01:10:48] error: /checkout/src/test/ui/conditional-compilation/cfg-attr-syntax-validation.rs:30: unexpected error: '30:25: 30:30: expected unsuffixed literal or identifier, found `concat!("nonexistent")`'
[01:10:48] 
[01:10:48] error: /checkout/src/test/ui/conditional-compilation/cfg-attr-syntax-validation.rs:30: expected error not found: expected unsuffixed literal or identifier, found concat!("nonexistent")
[01:10:48] error: 1 unexpected errors found, 1 expected errors not found
[01:10:48] status: exit code: 1
[01:10:48] status: exit code: 1
[01:10:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-syntax-validation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-syntax-validation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-syntax-validation/auxiliary" "-A" "unused"
[01:10:48]     Error {
[01:10:48]         line_num: 30,
[01:10:48]         kind: Some(
[01:10:48]             Error
[01:10:48]             Error
[01:10:48]         ),
[01:10:48]         msg: "30:25: 30:30: expected unsuffixed literal or identifier, found `concat!(\"nonexistent\")`"
[01:10:48] ]
[01:10:48] 
[01:10:48] not found errors (from test file): [
[01:10:48]     Error {
[01:10:48]     Error {
[01:10:48]         line_num: 30,
[01:10:48]         kind: Some(
[01:10:48]             Error
[01:10:48]         ),
[01:10:48]         msg: "expected unsuffixed literal or identifier, found concat!(\"nonexistent\")"
[01:10:48] ]
[01:10:48] 
[01:10:48] thread '[ui] ui/conditional-compilation/cfg-attr-syntax-validation.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1378:13
[01:10:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:10:48] 
[01:10:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:10:48] 
[01:10:48] 
[01:10:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:48] 
[01:10:48] 
[01:10:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:48] Build completed unsuccessfully in 0:04:25
[01:10:48] Build completed unsuccessfully in 0:04:25
[01:10:48] make: *** [check] Error 1
[01:10:48] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b8f3cc4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar  6 22:31:07 UTC 2019
---
travis_time:end:102371e0:start=1551911468464525716,finish=1551911468472534507,duration=8008791
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cfebaac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16041058
$ dmesg | grep -i kill
