plain
    100% |████████████████████████████████| 51kB 7.4MB/s 
Collecting botocore==1.10.51 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/ba/f6c9220d87784a85f24a8f2425edccb2f330d15c304ea2373ed8206a03ca/botocore-1.10.51-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 31.8MB/s eta 0:00:01
    0% |▏                               | 20kB 17.7MB/s eta 0:00:01
    0% |▎                               | 30kB 22.1MB/s eta 0:00:01
    0% |▎                               | 40kB 15.6MB/s eta 0:00:01
---
[00:01:18] Successfully tagged rust-ci:latest
[00:01:18] Built container sha256:df5a92b2105f4fdf3aac0e842aa48a2e2c9fee4eee10ab23db8e7d1547f24a8a
[00:01:18] Uploading finished image to s3://rust-lang-ci-sccache2/docker/9741addbd98d4a85d2c01f7aa80cb4e43a0c6bd2e3869e3a9643d4f5e63908f89c8ab1047e23987be8d46e850f6b8676d8af326da40164a6450bf4a21f787530
[00:01:18] 
[00:01:18] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:01:25] xargs: docker: terminated by signal 13

[00:01:25] travis_time:end:0151dc7c:start=1530831549266748062,finish=1530831622652151776,duration=73385403714
[CI_JOB_NAME=x86_64-gnu-llvm-3.9]
[00:01:25] [CI_JOB_NAME=x86_64-gnu-llvm-3.9]
---

[00:04:35] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:35] tidy error: /checkout/src/ci/docker/scripts/musl.sh:35: TODO is deprecated; use FIXME
[00:04:37] some tidy checks failed
[00:04:37] 
[00:04:37] 
[00:04:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:37] 
[00:04:37] 
[00:04:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:37] Build completed unsuccessfully in 0:00:49
[00:04:37] Build completed unsuccessfully in 0:00:49
[00:04:37] Makefile:79: recipe for target 'tidy' failed
[00:04:37] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1eccccc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00b290d4:start=1530831814925961857,finish=1530831814934479746,duration=8517889
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:206b4423
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06e94480
$ dmesg | grep -i kill
