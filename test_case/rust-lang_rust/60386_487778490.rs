plain
travis_time:end:235ca800:start=1556579489325486344,finish=1556579573266183522,duration=83940697178
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
DEPRECATION: Python 2.7 will reach the end of its life on January 1st, 2020. Please upgrade your Python as Python 2.7 won't be maintained after that date. A future version of pip will drop support for Python 2.7.
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/ec/78/01f03354bf02d4c3d1a0de01cbe77529b79df4ea078284f1172690834586/awscli-1.16.148-py2.py3-none-any.whl (1.5MB)
    1% |▍                               | 20kB 2.2MB/s eta 0:00:01
    2% |▋                               | 30kB 3.2MB/s eta 0:00:01
    2% |▉                               | 40kB 2.1MB/s eta 0:00:01
    3% |█                               | 51kB 2.6MB/s eta 0:00:01
---

[00:03:19] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:20] tidy error: /checkout/src/test/run-pass/cfg/cfg-family.rs: missing trailing newline
[00:03:20] tidy error: /checkout/src/test/run-pass/cfg/cfg-target-family.rs: missing trailing newline
[00:03:21] some tidy checks failed
[00:03:21] 
[00:03:21] 
[00:03:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:21] 
[00:03:21] 
[00:03:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:21] Build completed unsuccessfully in 0:00:46
[00:03:21] Build completed unsuccessfully in 0:00:46
[00:03:21] Makefile:67: recipe for target 'tidy' failed
[00:03:21] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e2b140c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 29 23:16:23 UTC 2019
---
travis_time:end:32f446b0:start=1556579784335267067,finish=1556579784340249342,duration=4982275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a09c5e8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0696f34e
travis_time:start:0696f34e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:24c89df9
$ dmesg | grep -i kill
