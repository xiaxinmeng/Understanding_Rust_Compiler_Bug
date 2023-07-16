plain
    100% |████████████████████████████████| 61kB 8.1MB/s 
Collecting botocore==1.10.5 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/79/66/5709a5eb3b4cb3e62a40ef514fefcf829a369a75854ed69f630fa27444b4/botocore-1.10.5-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 32.8MB/s eta 0:00:01
    0% |▏                               | 20kB 32.7MB/s eta 0:00:01
    0% |▎                               | 30kB 38.0MB/s eta 0:00:01
    0% |▎                               | 40kB 20.0MB/s eta 0:00:01
---

[00:04:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:46] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:538: line longer than 100 chars
[00:04:46] tidy error: /checkout/src/librustc_resolve/resolve_imports.rs:680: line longer than 100 chars
[00:04:46] tidy error: /checkout/src/librustc_resolve/lib.rs:2824: line longer than 100 chars
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:46
[00:04:47] Build completed unsuccessfully in 0:01:46
[00:04:47] make: *** [tidy] Error 1
[00:04:47] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06445788
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
