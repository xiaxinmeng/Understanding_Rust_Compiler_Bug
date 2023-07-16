plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/c2/1e/f70d1125f5bf6383d2ee7a76aea72bed6ba103c1bb9dd4ca051787552d2b/awscli-1.15.24-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 9.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.2MB/s eta 0:00:01
    2% |▉                               | 30kB 1.5MB/s eta 0:00:01
    3% |█                               | 40kB 1.3MB/s eta 0:00:01
---

[00:05:49] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:50] tidy error: /checkout/src/librustc_metadata/encoder.rs:1232: line longer than 100 chars
[00:05:50] tidy error: /checkout/src/librustdoc/html/render.rs:2606: line longer than 100 chars
[00:05:50] tidy error: /checkout/src/libsyntax/visit.rs:229: trailing whitespace
[00:05:50] tidy error: /checkout/src/libsyntax/test.rs:130: line longer than 100 chars
[00:05:50] tidy error: Found 1 features without a gate test.
[00:05:50] Expected a gate test for the feature 'async_await'.
[00:05:50] Hint: create a failing test file named 'feature-gate-async_await.rs'
[00:05:50]       in the 'ui' test suite, with its failures due to
[00:05:50]       missing usage of #![feature(async_await)].
[00:05:50] Hint: If you already have such a test and don't want to rename it,
[00:05:50]       you can also add a // gate-test-async_await line to the test file.
[00:05:51] some tidy checks failed
[00:05:51] 
[00:05:51] 
[00:05:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:51] 
[00:05:51] 
[00:05:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:51] Build completed unsuccessfully in 0:02:30
[00:05:51] Build completed unsuccessfully in 0:02:30
[00:05:51] make: *** [tidy] Error 1
[00:05:51] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:125a1855
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
