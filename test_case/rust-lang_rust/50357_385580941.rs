plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/3f/6c/dbbd5992740649134e597833bea5a95e1fc093a7123e9b8156d838b960e4/awscli-1.15.11-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 13.1MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 8.2MB/s 
Collecting botocore==1.10.11 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/15/c0/04ec8aec3cdf7dd4887f2666044550eb3370a4f29668e53519cc7143bdcf/botocore-1.10.11-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 40.0MB/s eta 0:00:01
    0% |▏                               | 20kB 13.1MB/s eta 0:00:01
    0% |▎                               | 30kB 17.2MB/s eta 0:00:01
    0% |▎                               | 40kB 12.4MB/s eta 0:00:01
---
[00:03:34]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:34]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:35]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:35]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:39] warning: unnecessary `unsafe` block
[00:03:39]    --> liballoc/arc.rs:990:9
[00:03:39] 990 |         unsafe {
[00:03:39] 990 |         unsafe {
[00:03:39]     |         ^^^^^^ unnecessary `unsafe` block
[00:03:39]     = note: #[warn(unused_unsafe)] on by default
[00:03:39] 
[00:03:40]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:57]     Finished release [optimized] target(s) in 54.63 secs
---

[00:04:58] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:58] tidy error: /checkout/src/liballoc/arc.rs:975: line longer than 100 chars
[00:04:58] tidy error: /checkout/src/liballoc/arc.rs:1105: line longer than 100 chars
[00:04:59] some tidy checks failed
[00:04:59] 
[00:04:59] 
[00:04:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:59] 
[00:04:59] 
[00:04:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:59] Build completed unsuccessfully in 0:01:58
[00:04:59] Build completed unsuccessfully in 0:01:58
[00:05:00] Makefile:79: recipe for target 'tidy' failed
[00:05:00] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2d9860e3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
