plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/78/5c/efd9fef62acc9fba7a0cb463673b14d537f45abd4f67bec998955b67fd7d/awscli-1.15.15-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▉                               | 30kB 2.0MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
    100% |████████████████████████████████| 1.3MB 944kB/s 
Collecting botocore==1.10.15 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/3e/a2/4eddf990705eccdda115d8e9e7c20fa21615704ed0806fc9eba6c2fcaad5/botocore-1.10.15-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 34.8MB/s eta 0:00:01
    0% |▏                               | 20kB 35.6MB/s eta 0:00:01
    0% |▎                               | 30kB 37.3MB/s eta 0:00:01
    0% |▎                               | 40kB 19.2MB/s eta 0:00:01
---

[00:04:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:51] tidy error: /checkout/src/librustc_trans/intrinsic.rs:1183: line longer than 100 chars
[00:04:51] tidy error: /checkout/src/librustc_trans/intrinsic.rs:1210: line longer than 100 chars
[00:04:51] tidy error: /checkout/src/librustc_trans/intrinsic.rs:1217: line longer than 100 chars
[00:04:51] tidy error: /checkout/src/librustc_trans/intrinsic.rs:1547: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/test/codegen/simd-intrinsic-generic-scatter.rs:36: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/test/codegen/simd-intrinsic-generic-scatter.rs:44: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/test/codegen/simd-intrinsic-generic-gather.rs:36: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/test/codegen/simd-intrinsic-generic-gather.rs:44: line longer than 100 chars
[00:04:53] some tidy checks failed
[00:04:53] 
[00:04:53] 
[00:04:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:53] 
[00:04:53] 
[00:04:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:53] Build completed unsuccessfully in 0:01:57
[00:04:53] Build completed unsuccessfully in 0:01:57
[00:04:53] make: *** [tidy] Error 1
[00:04:53] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:128db9eb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
