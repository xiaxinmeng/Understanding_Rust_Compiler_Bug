plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a4/28/107c5a3a2552fc0bfb65b6a90bd37c1406755bedc019e532b2570d610913/awscli-1.15.26-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 14.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---

[00:04:33] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:33] tidy error: /checkout/src/librustc_resolve/lib.rs:2255: TODO is deprecated; use FIXME
[00:04:33] tidy error: /checkout/src/librustc_resolve/lib.rs:3291: line longer than 100 chars
[00:04:34] some tidy checks failed
[00:04:34] 
[00:04:34] 
[00:04:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:34] 
[00:04:34] 
[00:04:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:34] Build completed unsuccessfully in 0:01:47
[00:04:34] Build completed unsuccessfully in 0:01:47
[00:04:35] make: *** [tidy] Error 1
[00:04:35] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2176b238
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
