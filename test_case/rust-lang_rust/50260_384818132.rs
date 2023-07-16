plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/26/a5/981fcedb0cba5cd167382878c92956229d14f0ac6846c71f2cd87906d474/awscli-1.15.10-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 9.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
  Downloading https://files.pythonhosted.org/packages/b7/8e/ddb32ddaabd431813e180ca224e844bab8ad42fbb47ee07553f0ec44cd86/colorama-0.3.7-py2.py3-none-any.whl
Collecting botocore==1.10.10 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/77/c9/a40ebce24bbab4c7986fccdac9dade097385ad2feae73dcc47d31a1b4dc8/botocore-1.10.10-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 40.9MB/s eta 0:00:01
    0% |▏                               | 20kB 37.8MB/s eta 0:00:01
    0% |▎                               | 30kB 43.9MB/s eta 0:00:01
    0% |▎                               | 40kB 20.0MB/s eta 0:00:01
---

[00:04:49] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:49] tidy error: /checkout/src/librustc_lint/builtin.rs: too many trailing newlines (3)
[00:04:50] some tidy checks failed
[00:04:50] 
[00:04:50] 
[00:04:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:50] 
[00:04:50] 
[00:04:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:50] Build completed unsuccessfully in 0:01:58
[00:04:50] Build completed unsuccessfully in 0:01:58
[00:04:50] Makefile:79: recipe for target 'tidy' failed
[00:04:50] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14991358
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
