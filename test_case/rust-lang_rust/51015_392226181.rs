plain
Requirement already satisfied: PyYAML<=3.12,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.10.28 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2e/91/f0870d4de8eb78897ce781f3ff22fc492bbb9849b5c91f26db20b125ef36/botocore-1.10.28-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 43.2MB/s eta 0:00:01
    0% |▏                               | 20kB 38.7MB/s eta 0:00:01
    0% |▎                               | 30kB 43.6MB/s eta 0:00:01
    0% |▎                               | 40kB 19.2MB/s eta 0:00:01
---

[00:05:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:51] tidy error: /checkout/src/librustc_typeck/check_unused.rs:156: line longer than 100 chars
[00:05:52] some tidy checks failed
[00:05:52] 
[00:05:52] 
[00:05:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:52] 
[00:05:52] 
[00:05:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:52] Build completed unsuccessfully in 0:02:29
[00:05:52] Build completed unsuccessfully in 0:02:29
[00:05:52] make: *** [tidy] Error 1
[00:05:52] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:057af42d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
