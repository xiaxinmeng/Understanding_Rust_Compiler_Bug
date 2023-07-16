plain
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.12.6 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/de/d3/170921ce269cc54770e28b506ad05c3059bdc1924766ba1beb41d7fdaadc/botocore-1.12.6-py2.py3-none-any.whl (4.7MB)
    0% |                                | 10kB 46.1MB/s eta 0:00:01
    0% |▏                               | 20kB 12.9MB/s eta 0:00:01
    0% |▏                               | 30kB 16.8MB/s eta 0:00:01
    0% |▎                               | 40kB 9.5MB/s eta 0:00:01
---
[00:57:05] ....................................................................................................
[00:57:08] .....................................................i..............................................
[00:57:10] ....................................................................................................
[00:57:13] ....................................................................................................
[00:57:16] .iiiiiiiii..........................................................................................
[00:57:22] ....................................................................................................
[00:57:25] ...................................................................................i................
[00:57:28] ....................................................................................................
[00:57:31] ......................................i.i..ii.......................................................
---
[01:26:46] travis_fold:end:stage0-linkchecker

[01:26:46] travis_time:end:stage0-linkchecker:start=1537323820485404312,finish=1537323822987049076,duration=2501644764

[01:26:57] std/future/trait.Future.html:70: broken link fragment `#tymethod.into_waker` pointing to `std/task/struct.LocalWaker.html`
[01:26:57] std/future/trait.Future.html:72: broken link fragment `#tymethod.wake` pointing to `std/task/struct.LocalWaker.html`
[01:28:51] core/future/trait.Future.html:70: broken link fragment `#tymethod.into_waker` pointing to `core/task/struct.LocalWaker.html`
[01:28:51] core/future/trait.Future.html:72: broken link fragment `#tymethod.wake` pointing to `core/task/struct.LocalWaker.html`
[01:29:38] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:29:38] 
[01:29:38] 
[01:29:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:29:38] expected success, got: exit code: 101
[01:29:38] expected success, got: exit code: 101
[01:29:38] 
[01:29:38] 
[01:29:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:38] Build completed unsuccessfully in 0:41:26
[01:29:38] make: *** [check] Error 1
[01:29:38] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3211fec2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1af50e42:start=1537323997525677596,finish=1537323997530040737,duration=4363141
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0edd426c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then
