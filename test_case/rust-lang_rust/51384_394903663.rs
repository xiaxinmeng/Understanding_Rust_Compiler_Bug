plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/5f/86/363f1249d0b62d7cce3cb8973fb6715b57ca75f8425d6c45fe5e129531a9/awscli-1.15.33-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.2MB/s eta 0:00:01
    1% |▌                               | 20kB 940kB/s eta 0:00:02
    2% |▉                               | 30kB 1.2MB/s eta 0:00:02
    3% |█                               | 40kB 1.3MB/s eta 0:00:01
---

[00:05:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:16] tidy error: /checkout/src/librustdoc/lib.rs:162: line longer than 100 chars
[00:05:17] some tidy checks failed
[00:05:17] 
[00:05:17] 
[00:05:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:17] 
[00:05:17] 
[00:05:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:17] Build completed unsuccessfully in 0:02:02
[00:05:17] Build completed unsuccessfully in 0:02:02
[00:05:17] make: *** [tidy] Error 1
[00:05:17] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26302b5e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
