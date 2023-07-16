plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/03/9d/a0b73320e4b9d776b0b68a67c7dbdc4115eb9eceff992d6b56222ba550d3/awscli-1.15.27-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 23.6MB/s eta 0:00:01
    1% |▌                               | 20kB 2.0MB/s eta 0:00:01
    2% |▉                               | 30kB 2.3MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 552kB 2.1MB/s 
Collecting botocore==1.10.27 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/0a/fc/5dbeb052f4b70346ad3eb1ac291d503c0a28ed7e0f806f7337bbb73e3b76/botocore-1.10.27-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 31.0MB/s eta 0:00:01
    0% |▏                               | 20kB 32.7MB/s eta 0:00:01
    0% |▎                               | 30kB 38.3MB/s eta 0:00:01
    0% |▎                               | 40kB 40.1MB/s eta 0:00:01
---

[00:05:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:56] tidy error: /checkout/src/librustc_passes/ast_validation.rs:178: line longer than 100 chars
[00:05:58] some tidy checks failed
[00:05:58] 
[00:05:58] 
[00:05:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:58] 
[00:05:58] 
[00:05:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:58] Build completed unsuccessfully in 0:02:35
[00:05:58] Build completed unsuccessfully in 0:02:35
[00:05:58] make: *** [tidy] Error 1
[00:05:58] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e3efc80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
