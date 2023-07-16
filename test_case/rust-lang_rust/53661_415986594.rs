plain
[00:01:37] sha256:4ce6eec60c5e994eefda6b986f14beb415c0e7e1717cdd560041f17bf77eb518
[00:01:37] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-llvm-5.0/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:01:37] Sending build context to Docker daemon  501.8kB
[00:01:37] Step 1/6 : FROM ubuntu:16.04
[00:01:38] received unexpected HTTP status: 503 Service Unavailable
[00:01:39] Sending build context to Docker daemon  501.8kB
[00:01:39] Step 1/6 : FROM ubuntu:16.04
[00:01:39] Step 1/6 : FROM ubuntu:16.04
[00:01:39] received unexpected HTTP status: 503 Service Unavailable
[00:01:41] Sending build context to Docker daemon  501.8kB
[00:01:41] Step 1/6 : FROM ubuntu:16.04
[00:01:41] Step 1/6 : FROM ubuntu:16.04
[00:01:41] received unexpected HTTP status: 503 Service Unavailable
[00:01:45] Sending build context to Docker daemon  501.8kB
[00:01:45] Step 1/6 : FROM ubuntu:16.04
[00:01:45] Step 1/6 : FROM ubuntu:16.04
[00:01:45] received unexpected HTTP status: 503 Service Unavailable
[00:01:49] Sending build context to Docker daemon  501.8kB
[00:01:49] Step 1/6 : FROM ubuntu:16.04
[00:01:49] Step 1/6 : FROM ubuntu:16.04
[00:01:49] received unexpected HTTP status: 503 Service Unavailable
travis_time:end:1d1593b5:start=1535220128883359671,finish=1535220238619769124,duration=109736409453

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0e1fd25f
---
travis_time:end:0351e184:start=1535220239129111675,finish=1535220239140841667,duration=11729992
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ac8b2d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11b13f4d
travis_time:start:11b13f4d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08cc31cc
$ dmesg | grep -i kill
