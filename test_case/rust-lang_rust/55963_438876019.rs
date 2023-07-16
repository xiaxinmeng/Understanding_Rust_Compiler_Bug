plain
travis_time:end:1aa988ec:start=1542242747549891468,finish=1542242748595885885,duration=1045994417
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.12.45 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ab/fa/5853b620519ee0f4fbc9c08c0598c6b51b09b09774f130e98df7d7ce4fce/botocore-1.12.45-py2.py3-none-any.whl (4.8MB)
    0% |                                | 10kB 40.9MB/s eta 0:00:01
    0% |▏                               | 20kB 41.9MB/s eta 0:00:01
    0% |▏                               | 30kB 47.0MB/s eta 0:00:01
    0% |▎                               | 40kB 30.5MB/s eta 0:00:01
---

[00:03:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:51] tidy error: /checkout/src/test/run-pass/mpsc_stress.rs:106: TODO is deprecated; use FIXME
[00:03:52] some tidy checks failed
[00:03:52] 
[00:03:52] 
[00:03:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:52] 
[00:03:52] 
[00:03:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:52] Build completed unsuccessfully in 0:00:47
[00:03:52] Build completed unsuccessfully in 0:00:47
[00:03:52] Makefile:79: recipe for target 'tidy' failed
[00:03:52] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e8fb270
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 15 00:49:50 UTC 2018
---
travis_time:end:3222a1bc:start=1542242990781261197,finish=1542242990786336876,duration=5075679
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10e6de20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:219237a2
travis_time:start:219237a2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ab1968c
$ dmesg | grep -i kill
