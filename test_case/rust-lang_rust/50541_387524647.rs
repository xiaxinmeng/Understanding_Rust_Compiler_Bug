plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/c9/6c/72524c483cd871e1d55fb3c3fa029605d17182c83de6e27ca143aed635a8/awscli-1.15.16-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.2MB/s eta 0:00:02
    2% |▉                               | 30kB 1.4MB/s eta 0:00:01
    3% |█                               | 40kB 1.3MB/s eta 0:00:01
---

[00:06:12] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:13] tidy error: /checkout/src/librustdoc/lib.rs:717: line longer than 100 chars
[00:06:14] some tidy checks failed
[00:06:14] 
[00:06:14] 
[00:06:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:14] 
[00:06:14] 
[00:06:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:14] Build completed unsuccessfully in 0:02:37
[00:06:14] Build completed unsuccessfully in 0:02:37
[00:06:14] make: *** [tidy] Error 1
[00:06:14] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a6966ff
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
