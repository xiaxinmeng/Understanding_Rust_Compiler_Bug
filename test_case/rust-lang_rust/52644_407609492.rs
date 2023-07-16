plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/cc/1a/bb0bc699b37a766736b0c07a7344b1b985deb16870e9d14c75110ae74256/awscli-1.15.64-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.3MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:57:13] 
[00:57:13] To learn more, run the command again with --verbose.
[00:57:13] 
[00:57:13] 
[00:57:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[00:57:13] 
[00:57:13] 
[00:57:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:13] Build completed unsuccessfully in 0:16:42
[00:57:13] Build completed unsuccessfully in 0:16:42
[00:57:13] make: *** [check] Error 1
[00:57:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09ea3ca6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
