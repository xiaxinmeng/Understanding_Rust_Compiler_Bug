plain
travis_time:end:05b93990:start=1548167229922750386,finish=1548167301305231579,duration=71382481193
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-debug
---
[00:01:10] Step 7/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --enable-debug       --enable-lld       --enable-lldb       --enable-optimize
[00:01:10]  ---> Running in 8f73d3d91389
[00:01:10] Removing intermediate container 8f73d3d91389
[00:01:10]  ---> cc2ddca93b9d
[00:01:10] Step 8/8 : ENV SCRIPT   python2.7 ../x.py build &&   python2.7 ../x.py test src/test/run-make-fulldeps --test-args clang
[00:01:10] Removing intermediate container a9c725490877
[00:01:10]  ---> bf67eb77a819
[00:01:10] Successfully built bf67eb77a819
[00:01:10] Successfully tagged rust-ci:latest
[00:01:10] Successfully tagged rust-ci:latest
[00:01:10] Built container sha256:bf67eb77a819908ce3d199751d776becb041807bbc6198b9c2ec5351c31c7937
[00:01:10] Uploading finished image to s3://rust-lang-ci-sccache2/docker/443850f43276f1b1220ffd0cf910f2f5a59531a55b07fe397c725d708ad26207600e68981d9be3f02cc20d364dbf6404db559718f4ee2b09acee812129e0468e
[00:01:42] upload failed: - to s3://rust-lang-ci-sccache2/docker/443850f43276f1b1220ffd0cf910f2f5a59531a55b07fe397c725d708ad26207600e68981d9be3f02cc20d364dbf6404db559718f4ee2b09acee812129e0468e Unable to locate credentials

[00:01:42] travis_time:end:07ac3814:start=1548167321659331995,finish=1548167412685253398,duration=91025921403
[CI_JOB_NAME=x86_64-gnu-debug]
[00:01:43] [CI_JOB_NAME=x86_64-gnu-debug]
---
[00:34:40] -- Looking for sys/resource.h - found
[00:34:40] -- Clang version: 8.0.0
[00:34:40] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG
[00:34:40] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG - Failed
[00:34:40] -- Could NOT find Z3 (missing:  Z3_LIBRARIES Z3_INCLUDE_DIR) (Required is exact version "4.7.1")
[00:34:41] CMake Error at /usr/share/cmake-3.5/Modules/FindPackageHandleStandardArgs.cmake:148 (message):
[00:34:41]   Could NOT find PythonLibs (missing: PYTHON_LIBRARIES PYTHON_INCLUDE_DIRS)
[00:34:41] Call Stack (most recent call first):
[00:34:41]   /usr/share/cmake-3.5/Modules/FindPackageHandleStandardArgs.cmake:388 (_FPHSA_FAILURE_MESSAGE)
[00:34:41]   /usr/share/cmake-3.5/Modules/FindPythonLibs.cmake:265 (FIND_PACKAGE_HANDLE_STANDARD_ARGS)
[00:34:41]   /checkout/src/tools/lldb/cmake/modules/LLDBConfig.cmake:181 (find_package)
[00:34:41] 
[00:34:41] 
[00:34:41] -- Configuring incomplete, errors occurred!
[00:34:41] -- Configuring incomplete, errors occurred!
[00:34:41] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
[00:34:41] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
[00:34:41] command did not execute successfully, got: exit code: 1
[00:34:41] 
[00:34:41] 
[00:34:41] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:34:41]  finished in 10.831
[00:34:41] travis_fold:end:llvm

[00:34:41] travis_time:end:llvm:start=1548169380711736177,finish=1548169391543474609,duration=10831738432
---
travis_time:end:04b08978:start=1548169392846867960,finish=1548169392853672676,duration=6804716
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c07d7d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bfb976c
travis_time:start:0bfb976c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1af8b019
$ dmesg | grep -i kill
