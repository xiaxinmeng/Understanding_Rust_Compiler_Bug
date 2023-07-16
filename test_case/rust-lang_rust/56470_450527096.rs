plain
travis_time:end:06986647:start=1546120330250125833,finish=1546120331314562290,duration=1064436457
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    100% |████████████████████████████████| 552kB 2.5MB/s 
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/7b/7c/c9386b82a25115cccf1903441bba3cbadcfae7b678a20167347fa8ded34c/pyasn1-0.4.5-py2.py3-none-any.whl (73kB)
    13% |████▌                           | 10kB 48.8MB/s eta 0:00:01
    27% |█████████                       | 20kB 48.1MB/s eta 0:00:01
    41% |█████████████▍                  | 30kB 53.4MB/s eta 0:00:01
    55% |█████████████████▉              | 40kB 54.2MB/s eta 0:00:01
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:20] 
[01:06:20] running 118 tests
[01:06:42] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:06:46] ......iii.i.....ii
[01:06:46] 
[01:06:46]  finished in 26.287
[01:06:46] travis_fold:end:test_debuginfo

---
[01:13:43] .....................................i.............................................................. 200/289
ain.rs:495:22
[01:14:30] 
[01:14:30] 
[01:14:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:30] 
[01:14:30] 
[01:14:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:30] Build completed unsuccessfully in 0:18:53
[01:14:30] Build completed unsuccessfully in 0:18:53
[01:14:30] make: *** [check] Error 1
[01:14:30] Makefile:48: recipe for target 'check' failed
25868 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
25868 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
24100 ./src/tools/lldb/packages
24096 ./src/tools/lldb/packages/Python
---
travis_time:end:23760f84:start=1546124811747961604,finish=1546124811753539638,duration=5578034
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:172fd8aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; 
