plain
travis_time:end:0d3b5923:start=1543967651052981376,finish=1543967720382763039,duration=69329781663
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 1.4MB 920kB/s 
Collecting botocore==1.12.59 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/21/cf/71dfc14692883aaf709bc1098a56770173a760a14b0b1cb74471609181be/botocore-1.12.59-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 33.9MB/s eta 0:00:01
    0% |▏                               | 20kB 35.3MB/s eta 0:00:01
    0% |▏                               | 30kB 40.0MB/s eta 0:00:01
    0% |▎                               | 40kB 28.6MB/s eta 0:00:01
---

[00:03:27] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:28] tidy error: /checkout/src/librustdoc/html/static/.eslintrc.js: missing trailing newline
[00:03:29] some tidy checks failed
[00:03:29] 
[00:03:29] 
[00:03:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:29] 
[00:03:29] 
[00:03:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:29] Build completed unsuccessfully in 0:00:56
[00:03:29] Build completed unsuccessfully in 0:00:56
[00:03:29] make: *** [tidy] Error 1
[00:03:29] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04c8d1ab
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec  4 23:58:58 UTC 2018
---
travis_time:end:14b98bd0:start=1543967939131769131,finish=1543967939137031639,duration=5262508
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06f3f64f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:025fa84a
travis_time:start:025fa84a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02d73df1
$ dmesg | grep -i kill
