plain
travis_time:end:04b00954:start=1544835015758491123,finish=1544835016969221473,duration=1210730350
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 61kB 9.5MB/s 
Collecting botocore==1.12.66 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/51/da/3ed787b6ca3d33f626c1ba4e014449825db0d557981c4bef71f886fb1424/botocore-1.12.66-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 44.4MB/s eta 0:00:01
    0% |▏                               | 20kB 38.9MB/s eta 0:00:01
    0% |▏                               | 30kB 32.8MB/s eta 0:00:01
    0% |▎                               | 40kB 36.8MB/s eta 0:00:01
---
[00:47:04] failures:
[00:47:04] 
[00:47:04] ---- [ui] ui/nll/issue-55394.rs stdout ----
[00:47:04] 
[00:47:04] error: /checkout/src/test/ui/nll/issue-55394.rs:21: unexpected error: '21:9: 21:20: lifetime may not live long enough'
[00:47:04] 
[00:47:04] error: /checkout/src/test/ui/nll/issue-55394.rs:21: expected error not found: lifetimes may not live long enough
[00:47:04] error: 1 unexpected errors found, 1 expected errors not found
[00:47:04] status: exit code: 1
[00:47:04] status: exit code: 1
[00:47:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55394.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/a" "-Crp_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:04] 
[00:47:04] 
[00:47:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:04] Build completed unsuccessfully in 0:03:49
[00:47:04] Build completed unsuccessfully in 0:03:49
[00:47:04] make: *** [check] Error 1
[00:47:04] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14a49a34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 15 01:37:31 UTC 2018
