plain
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.11.5 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/9e/ef/5d6d9995946379e018bca8353271a3d7068daf5892f4bcdddd59faded358/botocore-1.11.5-py2.py3-none-any.whl (4.6MB)
    0% |                                | 10kB 46.5MB/s eta 0:00:01
    0% |▏                               | 20kB 3.0MB/s eta 0:00:02
    0% |▏                               | 30kB 4.3MB/s eta 0:00:02
    0% |▎                               | 40kB 3.7MB/s eta 0:00:02
---
configure: processing command line
[00:01:56] configure: 
[00:01:56] configure: rust.dist-src        := False
[00:01:56] configure: llvm.link-shared     := True
[00:01:56] configure: rust.remap-debuginfo := True
[00:01:56] configure: rust.debug-assertions := True
[00:01:56] configure: llvm.assertions      := True
[00:01:56] configure: build.locked-deps    := True
[00:01:56] configure: llvm.ccache          := sccache
---
travis_fold:start:make-tidy
travis_time:start:00a9761c
make -j 4 tidy
[00:04:24]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:25] thread 'main' panicked at 'failed to find rust sha', libcore/option.rs:989:5
[00:04:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:25] Build completed unsuccessfully in 0:00:01
[00:04:25] Build completed unsuccessfully in 0:00:01
[00:04:25] make: *** [tidy] Error 1
[00:04:25] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:006a786c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:085f31be:start=1535663673272848064,finish=1535663673279712105,duration=6864041
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09fb3340
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:24ebd880
travis_time:start:24ebd880
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01b23418
$ dmesg | grep -i kill
