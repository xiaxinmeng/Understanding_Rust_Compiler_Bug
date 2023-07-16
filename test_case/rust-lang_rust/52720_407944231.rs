plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/1c/95/6bf862d284c9117eb50b3832cf39780248744e2bda3bbbcd9c5f12c489bf/awscli-1.15.65-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 9.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▊                               | 30kB 1.9MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:41:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/compiletest/Cargo.toml" "--features" "" "--message-format" "json"
[00:41:58] expected success, got: exit code: 101
[00:41:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:41:58] Build completed unsuccessfully in 0:00:22
[00:41:58] make: *** [check] Error 1
[00:41:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0109ac3a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
