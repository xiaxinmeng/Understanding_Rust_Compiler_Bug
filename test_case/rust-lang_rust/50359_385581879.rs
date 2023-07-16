plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/3f/6c/dbbd5992740649134e597833bea5a95e1fc093a7123e9b8156d838b960e4/awscli-1.15.11-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 6.5MB/s 
Collecting botocore==1.10.11 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/15/c0/04ec8aec3cdf7dd4887f2666044550eb3370a4f29668e53519cc7143bdcf/botocore-1.10.11-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 32.2MB/s eta 0:00:01
    0% |▏                               | 20kB 30.4MB/s eta 0:00:01
    0% |▎                               | 30kB 36.4MB/s eta 0:00:01
    0% |▎                               | 40kB 17.1MB/s eta 0:00:01
---

[00:05:06] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:06] tidy error: /checkout/src/librustc_trans/back/link.rs:88: TODO is deprecated; use FIXME
[00:05:08] some tidy checks failed
[00:05:08] 
[00:05:08] 
[00:05:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:08] 
[00:05:08] 
[00:05:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:08] Build completed unsuccessfully in 0:02:02
[00:05:08] Build completed unsuccessfully in 0:02:02
[00:05:08] Makefile:79: recipe for target 'tidy' failed
[00:05:08] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1282f59a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
