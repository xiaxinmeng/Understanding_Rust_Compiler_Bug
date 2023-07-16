plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4a/7d/9c0f35dc594b78137929523f9632ec649b69af2d30ca6a7a8c60d414b23a/awscli-1.15.9-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 29.8MB/s eta 0:00:01
    1% |▌                               | 20kB 2.0MB/s eta 0:00:01
    2% |▉                               | 30kB 2.3MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 6.2MB/s 
Collecting botocore==1.10.9 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/fe/13/45a8eeb5d78e2578b8e55df58e3921369efe51eaa57a9d2a36e7bef45bcc/botocore-1.10.9-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 43.6MB/s eta 0:00:01
    0% |▏                               | 20kB 32.6MB/s eta 0:00:01
    0% |▎                               | 30kB 39.4MB/s eta 0:00:01
    0% |▎                               | 40kB 12.8MB/s eta 0:00:01
---
[00:04:52] * scopeguard 
[00:04:52] some tidy checks failed
[00:04:52] 
[00:04:52] 
[00:04:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:52] 
[00:04:52] 
[00:04:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:52] Build completed unsuccessfully in 0:01:52
[00:04:52] Build completed unsuccessfully in 0:01:52
[00:04:52] Makefile:79: recipe for target 'tidy' failed
[00:04:52] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0420b354
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
