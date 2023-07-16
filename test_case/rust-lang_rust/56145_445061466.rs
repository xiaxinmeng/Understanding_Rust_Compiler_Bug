plain
travis_time:end:0479004c:start=1544134215075933908,finish=1544134217485299789,duration=2409365881
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ff/94/838f88e12e6d3aadb427955e657f3fe3c763ee1ad9290d4601a287fed7d1/awscli-1.16.71-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 40.5MB/s eta 0:00:01
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
472 ./src/libstd/sys/unix
468 ./src/test/ui/resolve
460 ./src/test/ui/rust-2018
448 ./src/test/ui/traits
448 ./src/test/ui/re_rebalance_coherence
444 ./src/test/ui/privacy
444 ./src/librustc_resolve
424 ./src/doc/unstable-book
416 ./src/test/ui/associated-types
---
[00:47:15] .................................................................................................... 1100/5183
[00:47:17] .................................................................................................... 1200/5183
[00:47:19] .................................................................................................... 1300/5183
[00:47:22] .................................................................................................... 1400/5183
[00:47:24] .................................................................................F.................. 1500/5183
[00:47:27] .............................i....................................................................i. 1600/5183
[00:47:34] .................................................................................................... 1800/5183
[00:47:37] .................................................................................................... 1900/5183
[00:47:41] .......................................i............................................................ 2000/5183
[00:47:45] .................................................................................................... 2100/5183
---
[00:48:38] ................................................i................................................... 3700/5183
[00:48:40] .................................................................................................... 3800/5183
[00:48:41] ...i................................................................................................ 3900/5183
[00:48:46] .................................................................................................... 4000/5183
[00:48:51] ...............................................................................F.................... 4100/5183
[00:48:57] .................................................................................................... 4300/5183
[00:49:01] ......................................................i............................................. 4400/5183
[00:49:06] .................................................................................................... 4500/5183
[00:49:09] .................................................................................................... 4600/5183
---
[00:49:24] .................................................................................................... 5100/5183
[00:49:26] ......................i............................................................
[00:49:26] failures:
[00:49:26] 
[00:49:26] ---- [ui] ui/feature-gates/feature-gate-re-rebalance-coherence.rs stdout ----
[00:49:26] 
[00:49:26] error: /checkout/src/test/ui/feature-gates/feature-gate-re-rebalance-coherence.rs:10: unexpected error: '10:1: 10:70: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`) [E0210]'
[00:49:26] error: 1 unexpected errors found, 0 expected errors not found
[00:49:26] status: exit code: 1
[00:49:26] status: exit code: 1
[00:49:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-re-rebalance-coher_rebalance_coherence/coherence-pair-covered-uncovered-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1/auxiliary" "-A" "unused"
[00:49:26]     Error {
[00:49:26]         line_num: 23,
[00:49:26]         kind: Some(
[00:49:26]             Error
[00:49:26]             Error
[00:49:26]         ),
[00:49:26]         msg: "23:1: 23:46: only traits defined in the current crate can be implemented for arbitrary types [E0117]"
[00:49:26] ]
[00:49:26] 
[00:49:26] 
[00:49:26] thread '[ui] ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:49:26] 
[00:49:26] failures:
[00:49:26] failures:
[00:49:26]     [ui] ui/feature-gates/feature-gate-re-rebalance-coherence.rs
[00:49:26]     [ui] ui/re_rebalance_coherence/coherence-pair-covered-uncovered-1.rs
[00:49:26] test result: FAILED. 5157 passed; 2 failed; 24 ignored; 0 measured; 0 filtered out
[00:49:26] 
[00:49:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:49:26] 
[00:49:26] 
[00:49:26] 
[00:49:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:26] 
[00:49:26] 
[00:49:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:26] Build completed unsuccessfully in 0:03:57
[00:49:26] Build completed unsuccessfully in 0:03:57
[00:49:26] make: *** [check] Error 1
[00:49:26] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13c72860
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec  6 22:59:53 UTC 2018
---
travis_time:end:06c75ad4:start=1544137194832883116,finish=1544137194838481555,duration=5598439
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d115745
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|
