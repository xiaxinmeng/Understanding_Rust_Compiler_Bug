plain
travis_time:end:0619a8bc:start=1540914687986726366,finish=1540914690389177275,duration=2402450909
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:01:43] sha256:4a689991aa24aeb0339a27a1b5f42d040f28b0411d37d4812816e79f7049516c
[00:01:43] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-llvm-5.0/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:01:43] Sending build context to Docker daemon  501.8kB
[00:01:43] Step 1/6 : FROM ubuntu:16.04
[00:01:43] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:45] Sending build context to Docker daemon  501.8kB
[00:01:45] Step 1/6 : FROM ubuntu:16.04
[00:01:45] Step 1/6 : FROM ubuntu:16.04
[00:01:45] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:47] Sending build context to Docker daemon  501.8kB
[00:01:47] Step 1/6 : FROM ubuntu:16.04
[00:01:47] Step 1/6 : FROM ubuntu:16.04
[00:01:47] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:50] Sending build context to Docker daemon  501.8kB
[00:01:50] Step 1/6 : FROM ubuntu:16.04
[00:01:50] Step 1/6 : FROM ubuntu:16.04
[00:01:51] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:55] Sending build context to Docker daemon  501.8kB
[00:01:55] Step 1/6 : FROM ubuntu:16.04
[00:01:55] Step 1/6 : FROM ubuntu:16.04
[00:01:55] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
travis_time:end:0050bd9b:start=1540914700594444642,finish=1540914816185057865,duration=115590613223

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0804e9e8
---
travis_time:end:00441784:start=1540914816608772143,finish=1540914816614833363,duration=6061220
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0109de02
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0420bf58
travis_time:start:0420bf58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:381e03b2
$ dmesg | grep -i kill
