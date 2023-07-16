plain
[00:01:27] sha256:4a689991aa24aeb0339a27a1b5f42d040f28b0411d37d4812816e79f7049516c
[00:01:27] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-llvm-5.0/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:01:27] Sending build context to Docker daemon  500.2kB
[00:01:27] Step 1/6 : FROM ubuntu:16.04
[00:01:27] received unexpected HTTP status: 503 Service Unavailable
[00:01:28] Sending build context to Docker daemon  500.2kB
[00:01:28] Step 1/6 : FROM ubuntu:16.04
[00:01:28] Step 1/6 : FROM ubuntu:16.04
[00:01:29] received unexpected HTTP status: 503 Service Unavailable
[00:01:31] Sending build context to Docker daemon  500.2kB
[00:01:31] Step 1/6 : FROM ubuntu:16.04
[00:01:31] Step 1/6 : FROM ubuntu:16.04
[00:01:31] received unexpected HTTP status: 503 Service Unavailable
[00:01:34] Sending build context to Docker daemon  500.2kB
[00:01:34] Step 1/6 : FROM ubuntu:16.04
[00:01:34] Step 1/6 : FROM ubuntu:16.04
[00:01:34] received unexpected HTTP status: 503 Service Unavailable
[00:01:38] Sending build context to Docker daemon  500.2kB
[00:01:38] Step 1/6 : FROM ubuntu:16.04
[00:01:38] Step 1/6 : FROM ubuntu:16.04
[00:01:39] received unexpected HTTP status: 503 Service Unavailable
travis_time:end:01a06e40:start=1540039290138299964,finish=1540039389241948914,duration=99103648950

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:17e5f900
---
travis_time:end:240491ac:start=1540039389678642097,finish=1540039389682859037,duration=4216940
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3716928c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10fd0b50
travis_time:start:10fd0b50
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0545435d
$ dmesg | grep -i kill
