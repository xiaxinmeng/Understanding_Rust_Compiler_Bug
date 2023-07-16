plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e0/2d/8dfbc3294db5b217570c41543549bb47feac371a122dde206bc542bb12ff/awscli-1.15.21-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 34.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.4MB/s eta 0:00:01
    2% |▉                               | 30kB 1.4MB/s eta 0:00:01
    3% |█                               | 40kB 1.5MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 6.8MB/s 
Collecting botocore==1.10.21 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/be/02/c225b57b9b775f9d70d5b70a6d950cdd7e3dd0ee7b0a6c8a322c90732f9d/botocore-1.10.21-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 29.0MB/s eta 0:00:01
    0% |▏                               | 20kB 26.2MB/s eta 0:00:01
    0% |▎                               | 30kB 29.8MB/s eta 0:00:01
    0% |▎                               | 40kB 30.3MB/s eta 0:00:01
---

[00:04:53] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:53] tidy error: /checkout/src/librustc_errors/emitter.rs:1420: line longer than 100 chars
[00:04:54] some tidy checks failed
[00:04:54] 
[00:04:54] 
[00:04:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:54] 
[00:04:54] 
[00:04:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:54] Build completed unsuccessfully in 0:01:54
[00:04:54] Build completed unsuccessfully in 0:01:54
[00:04:54] Makefile:79: recipe for target 'tidy' failed
[00:04:54] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1096024e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
