plain
travis_time:end:1d11d74b:start=1543457924660424368,finish=1543457925674514589,duration=1014090221
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 552kB 2.0MB/s 
Collecting botocore==1.12.55 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/d6/be/da181a69e355ec61224de70d2afd306d723834adf2af98ee163975cf8357/botocore-1.12.55-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 42.3MB/s eta 0:00:01
    0% |▏                               | 20kB 12.3MB/s eta 0:00:01
    0% |▏                               | 30kB 16.3MB/s eta 0:00:01
    0% |▎                               | 40kB 11.4MB/s eta 0:00:01
---

[00:03:00] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:00] tidy error: /checkout/src/test/rustdoc/synthetic_auto/self-referential.rs:38: line longer than 100 chars
[00:03:01] some tidy checks failed
[00:03:01] 
[00:03:01] 
[00:03:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:01] 
[00:03:01] 
[00:03:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:01] Build completed unsuccessfully in 0:00:54
[00:03:01] Build completed unsuccessfully in 0:00:54
[00:03:01] Makefile:79: recipe for target 'tidy' failed
[00:03:01] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1eb5ef82
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 29 02:21:56 UTC 2018
---
travis_time:end:0205c3a3:start=1543458117148252951,finish=1543458117152313537,duration=4060586
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e7f841c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0051fe3f
travis_time:start:0051fe3f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01dcf23c
$ dmesg | grep -i kill
