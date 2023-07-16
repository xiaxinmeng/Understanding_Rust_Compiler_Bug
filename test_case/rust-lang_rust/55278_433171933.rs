plain
travis_time:end:0cd83f04:start=1540494070261007312,finish=1540494125182596401,duration=54921589089
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
  Downloading https://files.pythonhosted.org/packages/b7/31/05c8d001f7f87f0f07289a5fc0fc3832e9a57f2dbd4d3b0fee70e0d51365/jmespath-0.9.3-py2.py3-none-any.whl
Collecting python-dateutil<3.0.0,>=2.1; python_version >= "2.7" (from botocore==1.12.31->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2f/e9/b02e8a1a8c53a55a4f37df1e8e111539d0a3e76828bcd252947a5200b797/python_dateutil-2.7.4-py2.py3-none-any.whl (211kB)
    4% |█▌                              | 10kB 43.9MB/s eta 0:00:01
    9% |███                             | 20kB 45.0MB/s eta 0:00:01
    14% |████▋                           | 30kB 48.2MB/s eta 0:00:01
    19% |██████▏                         | 40kB 48.8MB/s eta 0:00:01
---

[00:04:10] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:10] tidy error: /checkout/src/test/ui/consts/std/cell.rs: missing trailing newline
[00:04:10] tidy error: /checkout/src/test/ui/consts/std/char.rs: missing trailing newline
[00:04:10] tidy error: /checkout/src/test/ui/consts/std/iter.rs: missing trailing newline
[00:04:10] tidy error: /checkout/src/test/ui/consts/std/slice.rs: missing trailing newline
[00:04:11] some tidy checks failed
[00:04:11] 
[00:04:11] 
[00:04:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:11] 
[00:04:11] 
[00:04:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:11] Build completed unsuccessfully in 0:00:49
[00:04:11] Build completed unsuccessfully in 0:00:49
[00:04:11] Makefile:79: recipe for target 'tidy' failed
[00:04:11] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12f0ff23
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1a4f0aac:start=1540494387198512766,finish=1540494387202879678,duration=4366912
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:147d71e1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e39ffae
travis_time:start:0e39ffae
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0271b5b8
$ dmesg | grep -i kill
