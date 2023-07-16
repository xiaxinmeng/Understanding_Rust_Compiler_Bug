plain
[00:06:02] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-android/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:06:02] Sending build context to Docker daemon  500.2kB
[00:06:02] Step 1/14 : FROM ubuntu:16.04
[00:06:03] 16.04: Pulling from library/ubuntu
[00:06:03] received unexpected HTTP status: 503 Service Unavailable
[00:06:04] Sending build context to Docker daemon  500.2kB
[00:06:04] Step 1/14 : FROM ubuntu:16.04
[00:06:06] 16.04: Pulling from library/ubuntu
[00:06:06] 16.04: Pulling from library/ubuntu
[00:06:06] received unexpected HTTP status: 503 Service Unavailable
[00:06:08] Sending build context to Docker daemon  500.2kB
[00:06:08] Step 1/14 : FROM ubuntu:16.04
[00:06:08] Step 1/14 : FROM ubuntu:16.04
[00:06:08] received unexpected HTTP status: 503 Service Unavailable
[00:06:11] Sending build context to Docker daemon  500.2kB
[00:06:11] Step 1/14 : FROM ubuntu:16.04
[00:06:11] Step 1/14 : FROM ubuntu:16.04
[00:06:12] received unexpected HTTP status: 503 Service Unavailable
[00:06:16] Sending build context to Docker daemon  500.2kB
[00:06:16] Step 1/14 : FROM ubuntu:16.04
[00:06:16] Step 1/14 : FROM ubuntu:16.04
[00:06:16] received unexpected HTTP status: 503 Service Unavailable
travis_time:end:216fab7b:start=1540047314748081969,finish=1540047691370424816,duration=376622342847

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0a6b9cea
---
travis_time:end:013ccbec:start=1540047691788567450,finish=1540047691797130141,duration=8562691
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:216829e6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17e12200
travis_time:start:17e12200
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cd5917a
$ dmesg | grep -i kill
